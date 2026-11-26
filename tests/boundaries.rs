use serde_json::{json, Value};
use std::fs;
use std::os::unix::fs::{symlink, PermissionsExt};
use std::process::Command;
use std::time::{Duration, Instant};
use tempfile::tempdir;

fn invoke(root: &std::path::Path, args: &[&str], code: i32) -> Value {
    let out = Command::new(env!("CARGO_BIN_EXE_invar"))
        .current_dir(root)
        .args(args)
        .arg("--json")
        .output()
        .unwrap();
    assert!(out.stderr.is_empty(), "{out:?}");
    let value: Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(out.status.code(), Some(code), "{value}");
    assert_eq!(value["exit_code"], code);
    value
}

#[test]
fn ignored_writer_lock_is_recreated_but_verify_writes_nothing() {
    let d = tempdir().unwrap();
    let root = d.path();
    invoke(root, &["init"], 0);
    assert_eq!(
        fs::read(root.join(".invar/.gitignore")).unwrap(),
        b"candidate.json\nwriter.lock\n"
    );
    let hash = invoke(root, &["capture"], 0)["candidate_sha256"]
        .as_str()
        .unwrap()
        .to_owned();
    invoke(root, &["lock", "--candidate", &hash], 0);
    fs::remove_file(root.join(".invar/writer.lock")).unwrap();
    invoke(root, &["verify"], 0);
    assert!(!root.join(".invar/writer.lock").exists());
    invoke(root, &["capture"], 0);
    assert_eq!(
        fs::metadata(root.join(".invar/writer.lock"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o600
    );
    invoke(root, &["lock", "--candidate", &hash, "--replace"], 0);
}

#[test]
fn candidate_identity_and_configuration_drift_cannot_replace_acceptance() {
    let d = tempdir().unwrap();
    let root = d.path();
    invoke(root, &["init"], 0);
    let hash = invoke(root, &["capture"], 0)["candidate_sha256"]
        .as_str()
        .unwrap()
        .to_owned();
    invoke(root, &["lock", "--candidate", &hash], 0);
    let accepted = fs::read(root.join(".invar/accepted.json")).unwrap();
    let mut candidate = fs::read(root.join(".invar/candidate.json")).unwrap();
    candidate.push(b'\n');
    fs::write(root.join(".invar/candidate.json"), candidate).unwrap();
    invoke(root, &["lock", "--candidate", &hash, "--replace"], 3);
    let hash = invoke(root, &["capture"], 0)["candidate_sha256"]
        .as_str()
        .unwrap()
        .to_owned();
    fs::write(
        root.join("invar.json"),
        br#"{"version":1,"cases":[{"id":"hello","argv":["printf","different"]}]}"#,
    )
    .unwrap();
    invoke(root, &["lock", "--candidate", &hash, "--replace"], 3);
    assert_eq!(
        fs::read(root.join(".invar/accepted.json")).unwrap(),
        accepted
    );
}

#[test]
fn aggregate_snapshot_limit_and_case_limit_are_enforced() {
    let d = tempdir().unwrap();
    let root = d.path();
    invoke(root, &["init"], 0);
    invoke(root, &["capture"], 0);
    let candidate = fs::read(root.join(".invar/candidate.json")).unwrap();
    fs::write(
        root.join("streams.py"),
        "import os\nos.write(1,b'o'*1048576)\nos.write(2,b'e'*1048576)\n",
    )
    .unwrap();
    let cases: Vec<_> = (0..5)
        .map(|i| json!({"id":format!("case{i}"),"argv":["python3","streams.py"]}))
        .collect();
    fs::write(
        root.join("invar.json"),
        serde_json::to_vec(&json!({"version":1,"cases":cases})).unwrap(),
    )
    .unwrap();
    let result = invoke(root, &["capture"], 2);
    assert!(result["cases"][4]["message"]
        .as_str()
        .unwrap()
        .contains("8 MiB"));
    assert_eq!(
        fs::read(root.join(".invar/candidate.json")).unwrap(),
        candidate
    );
    let cases: Vec<_> = (0..33)
        .map(|i| json!({"id":format!("case{i}"),"argv":["true"]}))
        .collect();
    fs::write(
        root.join("invar.json"),
        serde_json::to_vec(&json!({"version":1,"cases":cases})).unwrap(),
    )
    .unwrap();
    invoke(root, &["capture"], 2);
}

#[test]
fn non_regular_files_and_symlink_destinations_fail_without_following_them() {
    use nix::sys::stat::Mode;
    use nix::unistd::mkfifo;
    let d = tempdir().unwrap();
    let root = d.path();
    invoke(root, &["init"], 0);
    fs::write(root.join("victim"), "preserve").unwrap();
    symlink("../victim", root.join(".invar/candidate.json")).unwrap();
    invoke(root, &["capture"], 2);
    assert_eq!(fs::read(root.join("victim")).unwrap(), b"preserve");
    fs::remove_file(root.join(".invar/candidate.json")).unwrap();
    let hash = invoke(root, &["capture"], 0)["candidate_sha256"]
        .as_str()
        .unwrap()
        .to_owned();
    symlink("../victim", root.join(".invar/accepted.json")).unwrap();
    invoke(root, &["lock", "--candidate", &hash, "--replace"], 2);
    assert_eq!(fs::read(root.join("victim")).unwrap(), b"preserve");
    fs::remove_file(root.join(".invar/accepted.json")).unwrap();
    mkfifo(
        &root.join(".invar/accepted.json"),
        Mode::S_IRUSR | Mode::S_IWUSR,
    )
    .unwrap();
    let before = Instant::now();
    invoke(root, &["verify"], 3);
    assert!(before.elapsed() < Duration::from_secs(2));
    fs::rename(root.join("invar.json"), root.join("real-config")).unwrap();
    symlink("real-config", root.join("invar.json")).unwrap();
    invoke(root, &["capture"], 2);
}

#[test]
fn exact_small_stream_limit_accepts_boundary_and_rejects_extra_byte() {
    let d = tempdir().unwrap();
    let root = d.path();
    invoke(root, &["init"], 0);
    fs::write(root.join("invar.json"), br#"{"version":1,"cases":[{"id":"tiny","argv":["python3","tiny.py"],"max_output_bytes":1}]}"#).unwrap();
    fs::write(
        root.join("tiny.py"),
        "import os\nos.write(1,b'x')\nos.write(2,b'y')\n",
    )
    .unwrap();
    let hash = invoke(root, &["capture"], 0)["candidate_sha256"]
        .as_str()
        .unwrap()
        .to_owned();
    invoke(root, &["lock", "--candidate", &hash], 0);
    invoke(root, &["verify"], 0);
    fs::write(root.join("tiny.py"), "import os\nos.write(1,b'xx')\n").unwrap();
    invoke(root, &["verify"], 2);
}
