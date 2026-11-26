import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tempfile
import time

parser = argparse.ArgumentParser(description="Guard Invar's real JSON interface while mutating an isolated source copy.")
parser.add_argument("--invar", type=Path, default=Path("target/debug/invar"))
args = parser.parse_args()
guard = args.invar.resolve(strict=True)
repo = Path(__file__).resolve().parents[1]
original_hashes = {p: hashlib.sha256(p.read_bytes()).digest() for p in (repo / "src").glob("*.rs")}
started = time.monotonic()

with tempfile.TemporaryDirectory(prefix="invar-dogfood-") as directory:
    work = Path(directory)
    source = work / "source"
    source.mkdir()
    for name in ("Cargo.toml", "Cargo.lock"):
        shutil.copyfile(repo / name, source / name)
    for name in ("src", "tests"):
        shutil.copytree(repo / name, source / name)
    build_env = dict(os.environ, CARGO_TARGET_DIR=str(work / "target"))
    subject = work / "target/debug/invar"

    def cargo(*arguments):
        result = subprocess.run(["cargo", *arguments, "--locked", "--offline"], cwd=source,
                                env=build_env, capture_output=True, text=True, timeout=180, check=True)
        return result.stdout

    def invoke(binary, project, *arguments, expected=0):
        result = subprocess.run([str(binary), "--project", str(project), *arguments, "--json"],
                                capture_output=True, timeout=10, check=False)
        assert result.returncode == expected, result.stdout
        assert not result.stderr, result.stderr
        report = json.loads(result.stdout)
        assert report["exit_code"] == expected
        return report

    def capture_candidate(project):
        captured = invoke(guard, project, "capture")
        raw = (project / ".invar/candidate.json").read_bytes()
        identity = hashlib.sha256(raw).hexdigest()
        assert captured["candidate_sha256"] == identity
        return raw, identity

    cargo("build", "--quiet")
    application = work / "application"
    application.mkdir()
    invoke(guard, application, "init")
    raw, identity = capture_candidate(application)
    snapshot = json.loads(raw)
    assert snapshot["config"]["cases"][0]["argv"] == ["printf", "hello\n"]
    assert snapshot["observations"][0]["exit_code"] == 0
    assert bytes.fromhex(snapshot["observations"][0]["stdout"]["hex"]) == b"hello\n"
    assert snapshot["observations"][0]["stderr"]["hex"] == ""
    invoke(guard, application, "lock", "--candidate", identity)

    contracts = work / "contracts"
    contracts.mkdir()
    invoke(guard, contracts, "init")
    argv = [str(subject), "--project", str(application), "verify", "--json"]
    (contracts / "invar.json").write_text(json.dumps({"version": 1, "cases": [
        {"id": "public-verify-json-v1", "argv": argv, "timeout_ms": 3000}
    ]}))
    raw, identity = capture_candidate(contracts)
    observation = json.loads(raw)["observations"][0]
    before = json.loads(bytes.fromhex(observation["stdout"]["hex"]))
    assert observation["exit_code"] == 0 and observation["stderr"]["hex"] == ""
    assert before["version"] == 1 and before["command"] == "verify" and before["status"] == "pass"
    print("BASELINE:", json.dumps(before), flush=True)
    invoke(guard, contracts, "lock", "--candidate", identity)
    assert invoke(guard, contracts, "verify")["status"] == "pass"
    accepted = (contracts / ".invar/accepted.json").read_bytes()
    print("BEFORE: CI reads report['command'] = verify; Invar PASS (exit 0)", flush=True)

    model = source / "src/model.rs"
    original = model.read_text()
    needle = "pub struct Report {\n    pub version: u32,\n    pub command: String,"
    assert original.count(needle) == 1
    model.write_text(original.replace(needle, needle.replace("    pub command:", '    #[serde(rename = "operation")]\n    pub command:')))
    print("INJECTED: JSON command -> operation; schema version unchanged; original repository untouched", flush=True)
    tests = cargo("test", "--all-targets", "--quiet")
    counts = re.findall(r"test result: ok\. (\d+) passed; 0 failed", tests)
    assert counts, tests
    print("EXISTING TESTS:", sum(map(int, counts)), "passed", flush=True)
    cargo("build", "--quiet")
    changed = invoke(subject, application, "verify")
    assert changed["version"] == 1 and changed["operation"] == "verify" and "command" not in changed
    try:
        changed["command"]
    except KeyError as error:
        print(f"CI CONSUMER: KeyError: {error}", flush=True)
    else:
        raise AssertionError("consumer unexpectedly accepted the incompatible report")
    regression = invoke(guard, contracts, "verify", expected=1)
    assert regression["cases"][0]["changes"][0]["channel"] == "stdout"
    print("INVAR: REGRESSION (exit 1)", flush=True)
    print(json.dumps(regression["cases"][0], indent=2), flush=True)
    assert (contracts / ".invar/accepted.json").read_bytes() == accepted
    assert (contracts / ".invar/candidate.json").read_bytes() == raw

    model.write_text(original)
    cargo("build", "--quiet")
    assert invoke(subject, application, "verify")["command"] == "verify"
    assert invoke(guard, contracts, "verify")["status"] == "pass"
    assert (contracts / ".invar/accepted.json").read_bytes() == accepted
    assert (contracts / ".invar/candidate.json").read_bytes() == raw
    assert all(hashlib.sha256(p.read_bytes()).digest() == digest for p, digest in original_hashes.items())
    print("REPAIRED: CI reads command again; Invar PASS (exit 0); accepted/candidate unchanged", flush=True)

print(f"DONE: {time.monotonic() - started:.3f}s; temporary copy removed={not work.exists()}; original Rust source unchanged")
