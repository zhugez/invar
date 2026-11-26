import argparse
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import tempfile
import time

parser = argparse.ArgumentParser(description="Exercise Invar's regression/repair workflow in a clean temporary project.")
parser.add_argument("--invar", type=Path, default=Path("target/debug/invar"))
args = parser.parse_args()
binary = args.invar.resolve(strict=True)
fixture = Path(__file__).resolve().parents[1] / "examples" / "ordering"
started = time.monotonic()
steps = []

with tempfile.TemporaryDirectory(prefix="invar-demo-") as directory:
    project = Path(directory)

    def invoke(*arguments, expected=0):
        command = [str(binary), *arguments, "--json"]
        completed = subprocess.run(command, cwd=project, capture_output=True, timeout=10, check=False)
        if completed.stderr:
            raise AssertionError(completed.stderr.decode(errors="backslashreplace"))
        report = json.loads(completed.stdout)
        if completed.returncode != expected or report["exit_code"] != expected:
            raise AssertionError(report)
        steps.append({"command": "invar " + " ".join(arguments), "exit": expected, "status": report["status"]})
        return report

    def conventional():
        completed = subprocess.run(["python3", "conventional_check.py"], cwd=project, capture_output=True, timeout=10, check=True)
        steps.append({"command": "python3 conventional_check.py", "exit": completed.returncode,
                      "stdout": completed.stdout.decode().strip()})

    invoke("init")
    for name in ["report.py", "conventional_check.py", "invar.json"]:
        shutil.copyfile(fixture / name, project / name)
    conventional()
    captured = invoke("capture")
    candidate = project / ".invar" / "candidate.json"
    candidate_bytes = candidate.read_bytes()
    identity = hashlib.sha256(candidate_bytes).hexdigest()
    assert identity == captured["candidate_sha256"]
    snapshot = json.loads(candidate_bytes)
    observation = snapshot["observations"][0]
    assert snapshot["config"]["cases"][0]["argv"] == ["python3", "report.py"]
    assert observation["exit_code"] == 0
    assert bytes.fromhex(observation["stdout"]["hex"]) == b"amber\nbirch\ncedar\n"
    assert observation["stderr"]["hex"] == ""
    steps.append({"review": "Checked argv, numeric exit, exact decoded stdout, empty stderr, and candidate SHA-256."})
    invoke("lock", "--candidate", identity)
    accepted_path = project / ".invar" / "accepted.json"
    accepted = accepted_path.read_bytes()
    invoke("verify")
    source = project / "report.py"
    original = source.read_text()
    source.write_text(original.replace("return sorted(NAMES)", "return sorted(NAMES, reverse=True)"))
    conventional()
    regression = invoke("verify", expected=1)
    assert regression["cases"][0]["changes"][0]["channel"] == "stdout"
    assert accepted_path.read_bytes() == accepted
    assert candidate.read_bytes() == candidate_bytes
    steps.append({"regression": regression["cases"][0], "accepted_and_candidate_unchanged": True})
    source.write_text(original)
    conventional()
    invoke("verify")
    assert accepted_path.read_bytes() == accepted

print(json.dumps({"version": 1, "steps": steps, "elapsed_seconds": round(time.monotonic() - started, 3),
                  "temporary_project_removed": not project.exists()}, indent=2))
