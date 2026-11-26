use crate::model::{working_dir, Blob, Case, Error, Observation, Result};
use nix::errno::Errno;
use nix::fcntl::{fcntl, FcntlArg, OFlag};
use nix::sys::signal::{killpg, Signal};
use nix::unistd::Pid;
use std::io::{self, Read};
use std::os::fd::AsFd;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::thread;
use std::time::{Duration, Instant};

struct Running {
    child: Child,
    pgid: Pid,
    armed: bool,
}

impl Running {
    fn terminate_group(&self) -> io::Result<()> {
        match killpg(self.pgid, Signal::SIGKILL) {
            Ok(()) | Err(Errno::ESRCH) => Ok(()),
            Err(e) => Err(io::Error::other(format!(
                "process group cleanup failed: {e}"
            ))),
        }
    }
}

impl Drop for Running {
    fn drop(&mut self) {
        if self.armed {
            let _ = self.terminate_group();
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}

fn nonblocking(fd: &impl AsFd) -> io::Result<()> {
    let flags = OFlag::from_bits_truncate(fcntl(fd, FcntlArg::F_GETFL)?);
    fcntl(fd, FcntlArg::F_SETFL(flags | OFlag::O_NONBLOCK))?;
    Ok(())
}

fn drain(pipe: &mut impl Read, bytes: &mut Vec<u8>, cap: usize) -> io::Result<bool> {
    let mut buffer = [0; 4096];
    for _ in 0..16 {
        match pipe.read(&mut buffer) {
            Ok(0) => return Ok(true),
            Ok(n) => {
                if n > cap - bytes.len() {
                    return Err(io::Error::other(
                        "output limit exceeded; observation was not captured",
                    ));
                }
                bytes.extend_from_slice(&buffer[..n]);
            }
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
            Err(e) if e.kind() == io::ErrorKind::Interrupted => break,
            Err(e) => return Err(e),
        }
    }
    Ok(false)
}

pub fn execute(root: &Path, case: &Case) -> Result<Observation> {
    let cwd = working_dir(root, case)?;
    let mut program = std::path::PathBuf::from(&case.argv[0]);
    if program.is_relative() && case.argv[0].contains('/') {
        program = cwd.join(program);
    }
    let mut command = Command::new(program);
    command
        .args(&case.argv[1..])
        .current_dir(cwd)
        .env_clear()
        .env(
            "PATH",
            std::env::var_os("PATH").unwrap_or_else(|| "/usr/bin:/bin".into()),
        )
        .env("LANG", "C")
        .env("LC_ALL", "C")
        .env("TZ", "UTC")
        .envs(&case.env)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .process_group(0);
    let start = Instant::now();
    let child = command
        .spawn()
        .map_err(|e| Error::execution(format!("spawn failed: {e}")))?;
    let pgid = Pid::from_raw(i32::try_from(child.id()).expect("Unix process ID fits i32"));
    let mut running = Running {
        child,
        pgid,
        armed: true,
    };
    let mut stdout = running.child.stdout.take().expect("piped stdout");
    let mut stderr = running.child.stderr.take().expect("piped stderr");
    let io_error = |e: io::Error| Error::execution(e.to_string());
    nonblocking(&stdout).map_err(io_error)?;
    nonblocking(&stderr).map_err(io_error)?;
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let (mut out_done, mut err_done) = (false, false);
    let mut status: Option<ExitStatus> = None;
    let mut exited_at = None;
    loop {
        if start.elapsed() >= Duration::from_millis(case.timeout_ms) {
            return Err(Error::execution(format!(
                "timeout after {} ms",
                case.timeout_ms
            )));
        }
        if !out_done {
            out_done = drain(&mut stdout, &mut out, case.max_output_bytes).map_err(io_error)?;
        }
        if !err_done {
            err_done = drain(&mut stderr, &mut err, case.max_output_bytes).map_err(io_error)?;
        }
        if status.is_none() {
            status = running.child.try_wait().map_err(io_error)?;
            if status.is_some() {
                running.terminate_group().map_err(io_error)?;
                exited_at = Some(Instant::now());
            }
        }
        if status.is_some() && out_done && err_done {
            break;
        }
        if exited_at.is_some_and(|t| t.elapsed() >= Duration::from_millis(100)) {
            return Err(Error::execution(
                "pipes remained open after process exit; an escaped descendant may hold them",
            ));
        }
        thread::sleep(Duration::from_millis(2));
    }
    running.armed = false;
    let exit_code = status
        .and_then(|s| s.code())
        .ok_or_else(|| Error::execution("command terminated by a signal"))?;
    Ok(Observation {
        id: case.id.clone(),
        exit_code,
        stdout: Blob::new(&out),
        stderr: Blob::new(&err),
    })
}
