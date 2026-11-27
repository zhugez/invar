import argparse
from pathlib import Path
import re
import subprocess
import tempfile

parser = argparse.ArgumentParser()
parser.add_argument("--invar", type=Path, required=True)
args = parser.parse_args()
binary = args.invar.resolve(strict=True)
guide = Path(__file__).resolve().parents[1] / "docs/first-run.md"
blocks = re.findall(r"^```sh\n(.*?)^```", guide.read_text(), re.M | re.S)
assert blocks and blocks[0].startswith("curl "), "Check installation separately before this exercise."
with tempfile.TemporaryDirectory(prefix="invar-first-run-check-") as directory:
    root = Path(directory)
    tools = root / "bin"
    tools.mkdir()
    (tools / "invar").symlink_to(binary)
    environment = {"HOME": str(root), "TMPDIR": str(root), "PATH": f"{tools}:/usr/bin:/bin", "LANG": "C"}
    result = subprocess.run(["/bin/sh", "-eu"], input="\n".join(blocks[1:]),
                            text=True, capture_output=True, cwd=root, env=environment, timeout=30)
    assert result.returncode == 0, (result.stdout, result.stderr)
    assert result.stdout.count("verify: pass") == 2, result.stdout
    assert "verify: regression" in result.stdout and "stdout changed" in result.stdout, result.stdout
    assert "Verification exit: 1 (expected 1 for a regression)" in result.stdout, result.stdout
    assert "Unexpected match" not in result.stdout, result.stdout
    assert sorted(p.name for p in root.iterdir()) == ["bin"], "The guide did not remove its disposable project."
    print("PASS: documented shell blocks, reviewed capture, pass -> regression -> pass, and cleanup")
