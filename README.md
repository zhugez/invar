# Invar

**Change code. Preserve behavior.**

Invar v0.1 is a local behavioral guardrail for coding agents and their maintainers. It records reviewed commands, requires explicit approval, and compares later runs with accepted behavior. It checks exit code, stdout, and stderr byte for byte.

It complements existing tests. A passing result covers the configured examples, not every possible behavior.

## Build

Requirements: Rust 1.88 or newer and Linux or macOS. Execution is verified on x86_64 Linux under WSL2. The macOS x86_64 compile check passes. Native macOS linking and execution are untested. Windows is unsupported.

From this repository:

```sh
cargo build --release --locked
INVAR_BIN="$PWD/target/release/invar"
"$INVAR_BIN" --version
```

Use that executable directly. For a local Cargo installation, run `cargo install --path . --locked` after checking for an existing `invar` executable.

The package is named `invar-behavior`, the executable is `invar`, and publication is disabled. Existing Invar names overlap. See the [naming review](docs/research/competitive-analysis.md).

## Try the complete workflow

Python 3 is required for this demonstration and the development tests. The core CLI requires only the commands you configure.

```sh
python3 scripts/demo.py --invar "$INVAR_BIN"
```

The script creates a clean temporary project and removes it afterward. It captures alphabetically sorted names, inspects the candidate, approves its exact hash, and verifies a pass. It then reverses the sort. The fixture's conventional membership/count checks still pass, while Invar reports a stdout regression. Restoring the sort returns verification to pass.

An ordinary ordering assertion would also catch this regression. The example demonstrates additional observed coverage, not superiority to conventional tests.

For a check against Invar's own CLI, run `python3 scripts/dogfood.py --invar "$INVAR_BIN"`. It builds an isolated copy offline and deliberately renames the public JSON field `command` to `operation`. The existing tests pass, but a representative Python CI consumer fails and the original Invar executable detects the changed output. Restoring the field returns verification to pass. The script removes its temporary source/build directories and checks that the original Rust source remains unchanged. This is an injected compatibility regression in the actual CLI, not evidence from an external customer project.

## Use your own project

This example creates a separate temporary project. Initialization executes nothing. Review `invar.json` before capture. The generated example invokes `printf` with an argument array.

```sh
INVAR_PROJECT="$(mktemp -d)"
"$INVAR_BIN" --project "$INVAR_PROJECT" init
python3 -m json.tool "$INVAR_PROJECT/invar.json"
"$INVAR_BIN" --project "$INVAR_PROJECT" capture
```

Inspect the captured configuration and all three result channels. This command displays escaped bytes and retains the identity of the exact file it reads:

```sh
INVAR_CANDIDATE_SHA256="$(python3 - "$INVAR_PROJECT" <<'PY'
import hashlib
import json
from pathlib import Path
import sys
raw = (Path(sys.argv[1]) / ".invar/candidate.json").read_bytes()
snapshot = json.loads(raw)
print(json.dumps(snapshot["config"], indent=2), file=sys.stderr)
for case in snapshot["observations"]:
    print(case["id"], "exit:", case["exit_code"], file=sys.stderr)
    print("stdout:", repr(bytes.fromhex(case["stdout"]["hex"])), file=sys.stderr)
    print("stderr:", repr(bytes.fromhex(case["stderr"]["hex"])), file=sys.stderr)
print(hashlib.sha256(raw).hexdigest())
PY
)"
```

After you approve what you inspected, execute:

```sh
"$INVAR_BIN" --project "$INVAR_PROJECT" lock --candidate "$INVAR_CANDIDATE_SHA256"
"$INVAR_BIN" --project "$INVAR_PROJECT" verify
"$INVAR_BIN" --project "$INVAR_PROJECT" verify --json
```

For an existing project, run `init` in that directory and replace the sample case with a reviewed command. Commands receive no stdin. They use the configured project-relative `cwd` and a cleared environment with PATH, LANG=C, LC_ALL=C, and TZ=UTC. Explicit `env` entries override those values. Arguments never receive implicit shell expansion. Name a shell explicitly in `argv` only when you intend shell execution.

Keep `invar.json`, `.invar/.gitignore`, and `.invar/accepted.json` under ordinary source review. The generated ignore file excludes transient candidates and the writer lock. Capture never accepts changes. Replacing an existing accepted snapshot requires a new reviewed capture and `lock --candidate "$INVAR_CANDIDATE_SHA256" --replace`.

Verification never writes accepted expectations, candidates, or configuration. Your configured commands can still change project files.

## Results and limits

`--json` emits one version-1 report. Case changes identify the channel, byte lengths, SHA-256 values, first differing offset, and escaped previews. Expected nonzero application exits can be approved. Spawn failures, signals, timeouts, and exceeded output limits are execution errors.

| Exit | Meaning |
| --- | --- |
| 0 | Success; verification matched all examples. |
| 1 | Behavior regression. |
| 2 | Configuration, execution, or infrastructure error. |
| 3 | Missing, invalid, or mismatched baseline. |

Configuration validation precedes baseline validation. Baseline failures execute no cases. During execution, errors take precedence over regressions. See the [specification](docs/specification.md) for formats and all limits.

Defaults: a one-second deadline and one MiB per stream, with configurable bounded limits. Suites allow at most 32 cases and eight MiB of aggregate captured output. Comparison is exact. No normalization, semantic JSON, or filesystem contracts exist yet.

**Trust is local.** Hashes detect corruption and pin a candidate. They do not authorize approval. An agent with sufficient write access can alter the baseline and recompute hashes. Protect the executable, configuration, baseline, and review process externally.

**Execution is not sandboxed.** Process groups clean up ordinary descendants. Escaped sessions require external containment. Commands can access files and networks. Do not run untrusted commands with privileges. See [SECURITY.md](SECURITY.md).

## Development and evidence

```sh
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
cargo +1.88.0 test --locked --all-targets
```

See [verification evidence](docs/verification.md), [contribution guidance](CONTRIBUTING.md), [architecture](docs/architecture.md), [research](docs/research/landscape.md), [roadmap](ROADMAP.md), and [engineering state](docs/engineering-state.md).

Apache-2.0. See [LICENSE](LICENSE).
