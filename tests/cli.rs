use serde_json::{json, Value};
use std::fs;
use std::os::unix::fs::{symlink, PermissionsExt};
use std::path::Path;
use std::process::{Command, Output};
use std::time::{Duration, Instant};
use tempfile::{tempdir, TempDir};

struct Project {
    dir: TempDir,
}

impl Project {
    fn new(script: &str) -> Self {
        let p = Self {
            dir: tempdir().unwrap(),
        };
        p.expect(&["init"], 0);
        p.write("fixture.py", script);
        p.config(json!([{"id":"sample","argv":["python3","fixture.py"]}]));
        p
    }
    fn root(&self) -> &Path {
        self.dir.path()
    }
    fn write(&self, path: &str, content: impl AsRef<[u8]>) {
        fs::write(self.root().join(path), content).unwrap();
    }
    fn read(&self, path: &str) -> Vec<u8> {
        fs::read(self.root().join(path)).unwrap()
    }
    fn config(&self, cases: Value) {
        self.write(
            "invar.json",
            serde_json::to_vec(&json!({"version":1,"cases":cases})).unwrap(),
        );
    }
    fn raw(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_invar"))
            .current_dir(self.root())
            .env("INVAR_PARENT_SECRET", "must-not-be-inherited")
            .args(args)
            .output()
            .unwrap()
    }
    fn expect(&self, args: &[&str], expected: i32) -> Value {
        let mut args = args.to_vec();
        args.push("--json");
        let out = self.raw(&args);
        assert!(out.stderr.is_empty(), "stderr: {:?}", out);
        let value: Value =
            serde_json::from_slice(&out.stdout).unwrap_or_else(|e| panic!("{e}: {out:?}"));
        assert_eq!(out.status.code(), Some(expected), "{value}");
        assert_eq!(value["exit_code"], expected, "{value}");
        assert_eq!(value["version"], 1);
        value
    }
    fn capture(&self) -> String {
        self.expect(&["capture"], 0)["candidate_sha256"]
            .as_str()
            .unwrap()
            .into()
    }
    fn approve(&self) -> String {
        let hash = self.capture();
        self.expect(&["lock", "--candidate", &hash], 0);
        hash
    }
    fn executable(&self, name: &str, body: &str) {
        self.write(name, body);
        fs::set_permissions(self.root().join(name), fs::Permissions::from_mode(0o700)).unwrap();
    }
}

#[test]
fn lifecycle_review_regression_repair_and_explicit_replacement() {
    let original = "import sys\nprint('alpha')\nprint('diagnostic',file=sys.stderr)\nsys.exit(7)\n";
    let p = Project::new(original);
    p.expect(&["verify"], 3);
    let hash = p.capture();
    assert!(!p.root().join(".invar/accepted.json").exists());
    p.expect(&["lock"], 2);
    p.expect(&["lock", "--candidate", &"0".repeat(64)], 3);
    p.expect(&["lock", "--candidate", &hash], 0);
    let accepted = p.read(".invar/accepted.json");
    let candidate = p.read(".invar/candidate.json");
    let config = p.read("invar.json");
    p.expect(&["verify"], 0);
    p.expect(&["lock", "--candidate", &hash], 2);
    p.write(
        "fixture.py",
        "import sys\nprint('beta')\nprint('changed',file=sys.stderr)\nsys.exit(8)\n",
    );
    let failure = p.expect(&["verify"], 1);
    let changes = failure["cases"][0]["changes"].as_array().unwrap();
    assert_eq!(
        changes
            .iter()
            .map(|c| c["channel"].as_str().unwrap())
            .collect::<Vec<_>>(),
        ["exit_code", "stdout", "stderr"]
    );
    assert_eq!(p.read(".invar/accepted.json"), accepted);
    assert_eq!(p.read(".invar/candidate.json"), candidate);
    assert_eq!(p.read("invar.json"), config);
    let newer = p.capture();
    assert_ne!(newer, hash);
    assert_eq!(p.read(".invar/accepted.json"), accepted);
    p.write("fixture.py", original);
    p.expect(&["verify"], 0);
    p.write(
        "fixture.py",
        "import sys\nprint('beta')\nprint('changed',file=sys.stderr)\nsys.exit(8)\n",
    );
    p.expect(&["lock", "--candidate", &hash, "--replace"], 3);
    p.expect(&["lock", "--candidate", &newer, "--replace"], 0);
    p.expect(&["verify"], 0);
}

#[test]
fn binary_output_and_actionable_offset_without_terminal_controls() {
    let p = Project::new("import sys\nsys.stdout.buffer.write(b'a'*100+b'\\x00\\xff\\x1b[31m')\n");
    p.approve();
    p.expect(&["verify"], 0);
    p.write(
        "fixture.py",
        "import sys\nsys.stdout.buffer.write(b'a'*100+b'\\x00\\xfe\\x1b[31m')\n",
    );
    let report = p.expect(&["verify"], 1);
    assert!(report["cases"][0]["changes"][0]["actual"]
        .as_str()
        .unwrap()
        .contains("offset=101"));
    let out = p.raw(&["verify"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(!out.stdout.contains(&0x1b));
    assert!(!out.stdout.contains(&0));
}

#[test]
fn malformed_configuration_and_semantic_drift_never_pass() {
    let p = Project::new("print('ok')");
    p.approve();
    let config = p.read("invar.json");
    let bad = [
        "{}", "{", "{\"version\":2,\"cases\":[]}",
        "{\"version\":1,\"cases\":[]}",
        "{\"version\":1,\"version\":1,\"cases\":[]}",
        "{\"version\":1,\"cases\":[{\"id\":\"x\",\"argv\":[\"true\"],\"env\":{\"A\":\"1\",\"A\":\"2\"}}]}",
    ];
    for b in bad {
        p.write("invar.json", b);
        p.expect(&["verify"], 2);
    }
    for case in [
        json!({"id":"x","argv":[]}),
        json!({"id":"x","argv":["true"],"surprise":true}),
        json!({"id":"x","argv":["true"],"timeout_ms":0}),
        json!({"id":"x","argv":["true"],"timeout_ms":30001}),
        json!({"id":"x","argv":["true"],"max_output_bytes":1048577}),
        json!({"id":"x","argv":["true"],"cwd":"../"}),
        json!({"id":"x","argv":["true"],"cwd":"/"}),
        json!({"id":"x","argv":["true"],"env":{"BAD=KEY":"x"}}),
        json!({"id":"x","argv":["true","\u{0}"]}),
    ] {
        p.config(json!([case]));
        p.expect(&["verify"], 2);
    }
    p.config(json!([{"id":"x","argv":["true"]},{"id":"x","argv":["true"]}]));
    p.expect(&["verify"], 2);
    p.config(json!([{"id":"sample","argv":["python3","fixture.py","changed"]}]));
    p.expect(&["verify"], 3);
    p.write("invar.json", config);
    p.expect(&["verify"], 0);
}

#[test]
fn snapshot_preflight_rejects_corruption_before_execution() {
    let p = Project::new("from pathlib import Path\nPath('executed').touch()\nprint('ok')");
    p.approve();
    let accepted = p.read(".invar/accepted.json");
    fs::remove_file(p.root().join("executed")).unwrap();
    let original: Value = serde_json::from_slice(&accepted).unwrap();
    let mut corruptions = vec![json!({"version":1}), original.clone()];
    corruptions[1]["observations"][0]["stdout"]["sha256"] = json!("invalid");
    for change in [
        "version", "order", "count", "exit", "hex", "config", "unknown",
    ] {
        let mut v = original.clone();
        match change {
            "version" => v["version"] = json!(2),
            "order" => v["observations"][0]["id"] = json!("wrong"),
            "count" => v["observations"] = json!([]),
            "exit" => v["observations"][0]["exit_code"] = json!(256),
            "hex" => v["observations"][0]["stdout"]["hex"] = json!("GG"),
            "config" => v["config_sha256"] = json!("invalid"),
            "unknown" => v["unexpected"] = json!(true),
            _ => unreachable!(),
        }
        corruptions.push(v);
    }
    for v in corruptions {
        p.write(".invar/accepted.json", serde_json::to_vec(&v).unwrap());
        p.expect(&["verify"], 3);
        assert!(!p.root().join("executed").exists());
    }
    p.write(".invar/accepted.json", "{");
    p.expect(&["verify"], 3);
    fs::remove_file(p.root().join(".invar/accepted.json")).unwrap();
    p.expect(&["verify"], 3);
    assert!(!p.root().join("executed").exists());
    p.write(".invar/accepted.json", &accepted);
    p.expect(&["verify"], 0);
}

#[test]
fn execution_errors_override_regressions_and_preserve_state() {
    let p = Project::new("print('before')");
    p.executable("tool", "#!/bin/sh\nprintf second\n");
    p.config(json!([
        {"id":"first","argv":["python3","fixture.py"]},
        {"id":"second","argv":["./tool"]}
    ]));
    p.approve();
    let accepted = p.read(".invar/accepted.json");
    let candidate = p.read(".invar/candidate.json");
    p.write("fixture.py", "print('after')");
    fs::remove_file(p.root().join("tool")).unwrap();
    let v = p.expect(&["verify"], 2);
    assert_eq!(v["cases"][0]["status"], "regression");
    assert_eq!(v["cases"][1]["status"], "error");
    p.expect(&["capture"], 2);
    assert_eq!(p.read(".invar/accepted.json"), accepted);
    assert_eq!(p.read(".invar/candidate.json"), candidate);
}

#[test]
fn streams_are_collected_together_and_output_caps_fail_without_publication() {
    let p = Project::new(
        "import os\nfor _ in range(100):\n os.write(1,b'o'*4096)\n os.write(2,b'e'*4096)\n",
    );
    p.approve();
    p.expect(&["verify"], 0);
    let candidate = p.read(".invar/candidate.json");
    p.write("fixture.py", "import os\nos.write(1,b'x'*(1048576+1))\n");
    let v = p.expect(&["capture"], 2);
    assert!(v["cases"][0]["message"]
        .as_str()
        .unwrap()
        .contains("output limit"));
    assert_eq!(p.read(".invar/candidate.json"), candidate);
    p.expect(&["verify"], 2);
    p.write("fixture.py", "import os\nos.write(2,b'x'*(1048576+1))\n");
    p.expect(&["capture"], 2);
}

#[test]
fn timeout_kills_same_group_descendants() {
    let p = Project::new("print('ok')");
    p.config(json!([{"id":"timeout","argv":["python3","fixture.py"],"timeout_ms":250}]));
    p.approve();
    p.write("fixture.py", "import subprocess,time\nfrom pathlib import Path\np=subprocess.Popen(['sleep','30'])\nPath('descendant.pid').write_text(str(p.pid))\ntime.sleep(30)\n");
    let before = Instant::now();
    let v = p.expect(&["verify"], 2);
    assert!(before.elapsed() < Duration::from_secs(3));
    assert!(v["cases"][0]["message"]
        .as_str()
        .unwrap()
        .contains("timeout"));
    let pid = String::from_utf8(p.read("descendant.pid")).unwrap();
    #[cfg(target_os = "linux")]
    {
        let stat = format!("/proc/{}/stat", pid.trim());
        for _ in 0..100 {
            let running = fs::read_to_string(&stat)
                .is_ok_and(|s| !s.split(") ").nth(1).unwrap().starts_with('Z'));
            if !running {
                return;
            }
            std::thread::sleep(Duration::from_millis(5));
        }
        panic!("descendant still running: {pid}");
    }
    #[cfg(target_os = "macos")]
    {
        let out = Command::new("ps")
            .args(["-o", "stat=", "-p", pid.trim()])
            .output()
            .unwrap();
        let status = String::from_utf8(out.stdout).unwrap();
        assert!(
            status.trim().is_empty() || status.trim().starts_with('Z'),
            "{status}"
        );
    }
}

#[test]
fn leader_exit_cleans_descendants_and_signal_exits_are_errors() {
    let p =
        Project::new("import subprocess\nsubprocess.Popen(['sleep','30'])\nprint('leader done')\n");
    let start = Instant::now();
    p.approve();
    assert!(start.elapsed() < Duration::from_secs(3));
    p.expect(&["verify"], 0);
    p.write(
        "fixture.py",
        "import os,signal\nos.kill(os.getpid(), signal.SIGTERM)\n",
    );
    let v = p.expect(&["verify"], 2);
    assert!(v["cases"][0]["message"]
        .as_str()
        .unwrap()
        .contains("signal"));
}

#[test]
fn environment_cwd_stdin_and_literal_arguments_follow_contract() {
    let p = Project::new("print('unused')");
    fs::create_dir(p.root().join("sub")).unwrap();
    p.write("sub/check.py", "import os,sys,json\nassert os.getcwd().endswith('/sub')\nassert 'INVAR_PARENT_SECRET' not in os.environ\nassert 'HOME' not in os.environ\nassert os.environ['INVAR_EXPLICIT']=='yes'\nassert os.environ['LC_ALL']=='C'\nassert os.environ['TZ']=='UTC'\nassert sys.stdin.read()==''\nassert sys.argv[1:]==['$(touch injected)', '*', '$HOME', 'a b']\nprint('checked')\n");
    p.config(json!([{"id":"env","argv":["python3","check.py","$(touch injected)","*","$HOME","a b"],"cwd":"sub","env":{"INVAR_EXPLICIT":"yes"}}]));
    p.approve();
    let snapshot: Value = serde_json::from_slice(&p.read(".invar/accepted.json")).unwrap();
    assert_eq!(snapshot["observations"][0]["exit_code"], 0);
    p.expect(&["verify"], 0);
    assert!(!p.root().join("sub/injected").exists());
    symlink("/", p.root().join("outside")).unwrap();
    p.config(json!([{"id":"escape","argv":["true"],"cwd":"outside"}]));
    p.expect(&["capture"], 2);
}

#[test]
fn sizes_symlinks_permissions_and_missing_state_are_not_trusted() {
    let p = Project::new("print('ok')");
    p.approve();
    assert_eq!(
        fs::metadata(p.root().join(".invar"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o700
    );
    assert_eq!(
        fs::metadata(p.root().join(".invar/accepted.json"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o600
    );
    fs::rename(
        p.root().join(".invar/accepted.json"),
        p.root().join("baseline"),
    )
    .unwrap();
    symlink("../baseline", p.root().join(".invar/accepted.json")).unwrap();
    p.expect(&["verify"], 3);
    fs::remove_file(p.root().join(".invar/accepted.json")).unwrap();
    let oversized = fs::File::create(p.root().join(".invar/accepted.json")).unwrap();
    oversized.set_len(20 * 1024 * 1024 + 1).unwrap();
    p.expect(&["verify"], 3);
    let config = p.read("invar.json");
    p.write("invar.json", vec![b' '; 256 * 1024 + 1]);
    p.expect(&["verify"], 2);
    p.write("invar.json", config);
    fs::rename(p.root().join(".invar"), p.root().join("saved-state")).unwrap();
    p.expect(&["verify"], 3);
    symlink("saved-state", p.root().join(".invar")).unwrap();
    p.expect(&["verify"], 3);
    p.expect(&["capture"], 2);
}

#[test]
fn cooperating_writers_fail_fast_and_verification_needs_no_writer_lock() {
    use nix::fcntl::{Flock, FlockArg};
    let p = Project::new("print('ok')");
    let hash = p.approve();
    let f = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(p.root().join(".invar/writer.lock"))
        .unwrap();
    let _lock = Flock::lock(f, FlockArg::LockExclusiveNonblock).unwrap();
    p.expect(&["capture"], 2);
    p.expect(&["lock", "--candidate", &hash, "--replace"], 2);
    p.expect(&["verify"], 0);
}

#[test]
fn init_refuses_existing_work_and_json_usage_errors_are_clean() {
    let p = Project::new("print('ok')");
    let config = p.read("invar.json");
    p.expect(&["init"], 2);
    assert_eq!(p.read("invar.json"), config);
    p.expect(&["unknown"], 2);
    p.expect(&["verify", "--unknown"], 2);
    let d = tempdir().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_invar"))
        .args(["--project", d.path().to_str().unwrap(), "init", "--json"])
        .output()
        .unwrap();
    assert!(output.status.success());
    assert!(d.path().join("invar.json").exists());
    let help = p.raw(&["--help"]);
    assert!(help.status.success());
    assert!(String::from_utf8(help.stdout).unwrap().contains("capture"));
}

#[test]
fn semantic_configuration_defaults_and_env_key_order_are_stable() {
    let p = Project::new("print('ok')");
    p.config(json!([{"id":"sample","argv":["python3","fixture.py"],"env":{"B":"2","A":"1"}}]));
    p.approve();
    p.write("invar.json", "{\"version\":1,\"cases\":[{\"id\":\"sample\",\"argv\":[\"python3\",\"fixture.py\"],\"cwd\":\".\",\"env\":{\"A\":\"1\",\"B\":\"2\"},\"timeout_ms\":1000,\"max_output_bytes\":1048576}]}");
    p.expect(&["verify"], 0);
}
