use crate::model::{CONFIG_LIMIT, SNAPSHOT_LIMIT};
use nix::fcntl::{Flock, FlockArg};
use std::fs::{self, DirBuilder, File, OpenOptions};
use std::io::{self, Read, Write};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

pub fn state(root: &Path) -> io::Result<PathBuf> {
    let path = root.join(".invar");
    if !fs::symlink_metadata(&path)?.file_type().is_dir() {
        return Err(invalid(".invar must be a real directory, not a symlink"));
    }
    Ok(path)
}

fn open_regular(path: &Path, write: bool) -> io::Result<File> {
    let file = OpenOptions::new()
        .read(true)
        .write(write)
        .create(write)
        .mode(0o600)
        .custom_flags(nix::libc::O_NOFOLLOW | nix::libc::O_NONBLOCK)
        .open(path)?;
    if !file.metadata()?.file_type().is_file() {
        return Err(invalid("expected a regular file"));
    }
    Ok(file)
}

fn read_bounded(path: &Path, limit: u64) -> io::Result<Vec<u8>> {
    let file = open_regular(path, false)?;
    if file.metadata()?.len() > limit {
        return Err(invalid("file exceeds size limit"));
    }
    let mut bytes = Vec::new();
    file.take(limit + 1).read_to_end(&mut bytes)?;
    if bytes.len() as u64 > limit {
        return Err(invalid("file exceeds size limit"));
    }
    Ok(bytes)
}

pub fn read_config(root: &Path) -> io::Result<Vec<u8>> {
    read_bounded(&root.join("invar.json"), CONFIG_LIMIT)
}
pub fn read_snapshot(state: &Path, name: &str) -> io::Result<Vec<u8>> {
    read_bounded(&state.join(name), SNAPSHOT_LIMIT)
}

pub fn writer_lock(state: &Path) -> io::Result<Flock<File>> {
    let file = open_regular(&state.join("writer.lock"), true)?;
    Flock::lock(file, FlockArg::LockExclusiveNonblock).map_err(|(_, e)| {
        io::Error::other(format!(
            "another writer holds the lock, or locking failed: {e}"
        ))
    })
}

pub fn write_atomic(path: &Path, bytes: &[u8], replace: bool) -> io::Result<()> {
    match fs::symlink_metadata(path) {
        Ok(m) if !m.file_type().is_file() => {
            return Err(invalid("refusing non-regular destination"))
        }
        Ok(_) => {}
        Err(e) if e.kind() == io::ErrorKind::NotFound => {}
        Err(e) => return Err(e),
    }
    let parent = path
        .parent()
        .ok_or_else(|| invalid("destination has no parent"))?;
    let mut file = NamedTempFile::new_in(parent)?;
    file.write_all(bytes)?;
    file.as_file().sync_all()?;
    if replace {
        file.persist(path).map_err(|e| e.error)?;
    } else {
        file.persist_noclobber(path).map_err(|e| e.error)?;
    }
    Ok(())
}

pub fn init(root: &Path, config: &[u8]) -> io::Result<()> {
    for path in [root.join("invar.json"), root.join(".invar")] {
        match fs::symlink_metadata(path) {
            Ok(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::AlreadyExists,
                    "invar.json or .invar already exists",
                ))
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {}
            Err(e) => return Err(e),
        }
    }
    let state = root.join(".invar");
    DirBuilder::new().mode(0o700).create(&state)?;
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(state.join("writer.lock"))?;
    write_atomic(
        &state.join(".gitignore"),
        b"candidate.json\nwriter.lock\n",
        false,
    )?;
    write_atomic(&root.join("invar.json"), config, false)
}
