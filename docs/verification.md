# Verification evidence

Recorded on 2026-11-26 from actual commands in this session. These are local engineering checks, not independent certification or user research.

## Environment

- x86_64 Linux, kernel 6.18.58.1-microsoft-standard-WSL2.
- Rust 1.100.0 (0ef1bc386, 2026-11-09).
- Minimum-toolchain check: Rust 1.88.0 (6b00bc388, 2025-06-23).
- Python 3.12.3 for integration fixtures and the demonstration.
- Cargo.lock records the resolved graph. First-party Rust forbids unsafe code.

## Checks and observed results

| Command or check | Observed result |
| --- | --- |
| cargo fmt --check | Passed. |
| cargo clippy --locked --all-targets -- -D warnings | Passed without warnings. |
| cargo test --locked --all-targets | 19 passed: one CLI-definition check, five boundary tests, thirteen lifecycle/failure tests. No skipped tests. |
| cargo +1.88.0 test --locked --all-targets | The same 19 tests passed on the declared minimum Rust version. |
| cargo build --release --locked | Passed. Release build during the README exercise took 9.05 seconds according to Cargo. |
| cargo check --locked --target x86_64-apple-darwin | Passed after installing the target standard library. This does not verify native linking or execution. |
| python3 scripts/demo.py --invar target/debug/invar | Passed the full regression/repair workflow in a clean temporary project. |
| Source installation | cargo install --path . --locked --root TEMP_DIR succeeded. Installed executable reported invar 0.1.0. Temporary install directory removed. |
| cargo metadata --locked --format-version 1 | Reviewed dependency names, versions, licenses, and declared minimum Rust versions. |
| cargo audit --version | Unavailable: no cargo-audit subcommand. No advisory-scan claim is made. |

The tests include expected nonzero exits, all three changed channels, binary bytes, first-change previews, stale/malformed/oversized/corrupt inputs, missing executables, read-only verification, mixed-result precedence, simultaneous pipes, timeouts, descendant termination, signal exits, environment clearing, literal arguments, cwd confinement, permissions, writer contention, cloned-state lock recreation, FIFO inputs, symlink reads/writes, case count, and per-stream/aggregate byte limits.

Review found that transient writer locks needed safe recreation after a clone. That defect is fixed and covered. Initialization creates a local ignore file for candidates and the writer lock. Verification does not create the missing lock.

## Demonstration transcript

The fixture promises alphabetically sorted names. Its conventional check covers membership and count, deliberately omitting order.

| Step | Actual outcome |
| --- | --- |
| init and conventional check | Exit 0. |
| capture | Exit 0. Accepted snapshot did not yet exist. |
| Review | Asserted argv, numeric exit 0, stdout amber/birch/cedar, empty stderr, and candidate identity. |
| lock --candidate REVIEWED_HASH | Exit 0. Explicit approval. |
| verify | Exit 0, pass. |
| Reverse sorting in report.py | Conventional membership/count check still exited 0. |
| verify | Exit 1, stdout regression, first differing byte 0. Expected amber/birch/cedar; actual cedar/birch/amber. |
| State check | Candidate and accepted bytes unchanged. |
| Restore sorting, conventional check, verify | All exited 0. Verification passed. |
| Cleanup | Temporary project removed. |

Candidate identity in the recorded demonstration: 542c4e42880d3ba5d3b2f8f45bb03e1f509cd4a72c087e10bf57bbbfd16b08e9. This is evidence of that candidate, not a permanent approval token.

## Invar checks its own JSON interface

Ran `python3 scripts/dogfood.py --invar target/debug/invar` against an isolated copy of the actual Rust CLI. The original executable guarded a real `verify --json` invocation. The candidate was inspected and approved before mutation.

Added a Serde rename from `command` to `operation` in the copy, without changing schema version 1. All 19 existing tests still passed. A representative Python CI consumer accessing `report["command"]` raised `KeyError: 'command'`. The original executable reported a stdout regression with exit 1 at byte offset 14. Restoring the original field returned both the consumer and verification to pass.

The final run took 9.267 seconds with cached registry dependencies and offline builds. Candidate/accepted bytes and the original Rust source stayed unchanged. The temporary source and build directories were removed. This is deliberate fault injection in a real CLI, not a naturally discovered production defect or an external customer evaluation. Adding a conventional assertion for this field would also detect the change.

## Onboarding measurement

Extracted all six sh blocks from README.md and executed them in sequence with bash -euo pipefail. Set TMPDIR to an outer TemporaryDirectory so the generated project was clean and cleanup was automatic. The run included the release build, demonstration, manual-workflow commands, escaped candidate inspection, explicit lock, human/JSON verification, formatting, Clippy, and tests on both toolchains.

Observed total: **13.245 seconds**, below the 300-second product target for this scripted engineering exercise. The release demonstration itself reported 0.129 seconds. An earlier debug demonstration reported 0.135 seconds.

Rust, Python, and registry downloads were already available. The release profile compiled during this run. Human reading, installation of development tools, fresh registry download time, and independent usability were not measured. Do not present this as a cold-start benchmark or proof that new users complete onboarding within five minutes.

## Review scope and remaining gaps

Reviewed subprocess lifecycle, alternating nonblocking reads, cleanup/error paths, strict parsing, digest validation, phased exit precedence, approval identity, atomic storage, symlink/FIFO handling, and JSON/terminal escaping. Formatting and whitespace checks cover the changed source set. The existing Apache-2.0 license remains unchanged. Pre-existing .pi state is preserved.

Native macOS execution is untested. Windows is unsupported. No external penetration test, advisory audit, customer trial, broad benchmark, or trademark clearance occurred. Google search was unavailable because the research authentication path was unavailable. Primary project documentation and Cargo metadata supplied the bounded research instead.

No Git commit, push, package publication, release, or deployment occurred. See [engineering state](engineering-state.md) and [security limitations](../SECURITY.md).
