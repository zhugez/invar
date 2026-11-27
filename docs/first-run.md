# Catch your first output regression

For maintainers using coding agents to change CLI tools or repository scripts. This walkthrough uses a disposable project, not your own code. It demonstrates one exact output contract: **pass → regression → pass**. It does not prove other behavior correct.

## 1. Install and check the executable

The prebuilt installer supports **Linux/WSL x86_64**. Review [install.sh](../install.sh), then run:

```sh
curl -fsSL https://github.com/zhugez/invar/raw/main/install.sh | sh
```

Open a new terminal if the installer changed PATH. In that terminal:

```sh
command -v invar
invar --version
```

Expected version: `invar 0.1.0`. An existing unrelated executable is never replaced. If installation reports a collision, use the [manual download](releases/v0.1.0.md#manual-download) in a separate directory and use its absolute executable path. Do not delete the existing tool.

The exercise below also needs Python 3 and a POSIX shell. It needs no Rust, Git, source checkout, account, or network after installation. Native macOS execution is unverified; [source-build instructions](getting-started.md#build-from-source) describe its current limits. Native Windows is unsupported.

## 2. Make a harmless example

Run the following blocks in the same shell. `INVAR_TRIAL` names a new temporary directory; it never reuses your project.

```sh
INVAR_TRIAL="$(mktemp -d)"
export INVAR_TRIAL
invar --project "$INVAR_TRIAL" init
python3 - <<'PY'
import json, os
from pathlib import Path
p = Path(os.environ['INVAR_TRIAL'])
(p / 'report.py').write_text('print("alice\\nbob\\ncarol")\n')
(p / 'invar.json').write_text(json.dumps({
    'version': 1,
    'cases': [{'id': 'names', 'argv': ['python3', 'report.py']}]
}, indent=2) + '\n')
print((p / 'invar.json').read_text())
PY
```

Read the printed configuration. It runs only `python3 report.py` in the temporary project. Configured commands have your filesystem and network permissions; Invar does not sandbox them.

## 3. Capture, inspect, and approve

```sh
invar --project "$INVAR_TRIAL" capture
```

Capture creates a candidate. It does **not** approve a baseline. Read the configuration, numeric exit code, and both decoded streams from the exact candidate bytes being hashed:

```sh
REVIEWED_SHA256="$(python3 - <<'PY'
import hashlib, json, os, sys
from pathlib import Path
raw = (Path(os.environ['INVAR_TRIAL']) / '.invar/candidate.json').read_bytes()
snapshot = json.loads(raw)
print(json.dumps(snapshot['config'], indent=2), file=sys.stderr)
for case in snapshot['observations']:
    print(case['id'], 'exit:', case['exit_code'], file=sys.stderr)
    print('stdout:', repr(bytes.fromhex(case['stdout']['hex'])), file=sys.stderr)
    print('stderr:', repr(bytes.fromhex(case['stderr']['hex'])), file=sys.stderr)
print(hashlib.sha256(raw).hexdigest())
PY
)"
```

Expect exit `0`, stdout `b'alice\nbob\ncarol\n'`, and stderr `b''`. Confirm the command arguments above as well. Only after you approve what you inspected, run:

```sh
invar --project "$INVAR_TRIAL" lock --candidate "$REVIEWED_SHA256"
invar --project "$INVAR_TRIAL" verify
```

Expected: verification passes, exit `0`. The hash identifies a candidate's bytes; it does not authenticate a reviewer.

## 4. Introduce an ordering regression

```sh
python3 - <<'PY'
import os
from pathlib import Path
(Path(os.environ['INVAR_TRIAL']) / 'report.py').write_text('print("carol\\nbob\\nalice")\n')
PY
if invar --project "$INVAR_TRIAL" verify; then
    printf 'Unexpected match: check that you changed the trial report.\n'
else
    INVAR_RESULT=$?
    printf 'Verification exit: %s (expected 1 for a regression)\n' "$INVAR_RESULT"
    test "$INVAR_RESULT" -eq 1
fi
```

Expected: `stdout changed` and exit `1`. Do not recapture and approve this changed output to make verification green. Restore the code when the original ordering is the intended contract. An ordinary ordering assertion could also detect this example.

## 5. Repair and verify again

```sh
python3 - <<'PY'
import os
from pathlib import Path
(Path(os.environ['INVAR_TRIAL']) / 'report.py').write_text('print("alice\\nbob\\ncarol")\n')
PY
invar --project "$INVAR_TRIAL" verify
```

Expected: pass, exit `0`. You changed the code back; the accepted baseline stayed the same.

When finished, remove only the disposable directory created by this walkthrough:

```sh
python3 - <<'PY'
import os, shutil
shutil.rmtree(os.environ['INVAR_TRIAL'])
PY
unset INVAR_TRIAL REVIEWED_SHA256 INVAR_RESULT
```

## Apply it to one real command

Choose a deterministic, harmless command whose output has a real consumer. Keep existing tests. Start with one reviewed case, then run `invar verify` after an actual code change. Track `invar.json`, `.invar/.gitignore`, and `.invar/accepted.json` in source review. Keep secrets and private output out of these files.

Exact comparison will flag timestamps, random identifiers, and other variable bytes too. Review those constraints before choosing a command. Invar currently does not normalize output or compare JSON semantically. See [project setup and limits](getting-started.md#use-your-own-project).

## Use Pi, or tell us where you got stuck

Pi integration currently loads from a source checkout. See the [Pi setup guide](../integrations/pi/README.md). New configurations offer detected pnpm, bun, Cargo, and common Justfile commands. Existing `invar.json` uses the configuration review flow. Detection executes nothing; capture and baseline approval require separate confirmations.

[Share first-use feedback](https://github.com/zhugez/invar/issues/new?template=first-use.yml), or [open an ordinary issue](https://github.com/zhugez/invar/issues/new) if the form is unavailable. Include your platform, Invar version, stopping point, and a synthetic reproduction. Do not upload raw snapshots, credentials, or private command output. This is early software with no support-response SLA. For security reports, follow [SECURITY.md](../SECURITY.md).
