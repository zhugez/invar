# Engineering state

Updated: 2026-11-27. Package: invar-behavior 0.1.0. Binary: invar. License: Apache-2.0. crates.io publication disabled.

## Delivered

M0 research/specification and the M1 command-boundary slice exist. init creates configuration and private local state. capture publishes only complete candidates. lock requires the reviewed candidate's hash and explicit replacement permission. verify preflights the complete baseline and never writes state. All three channels compare exactly, with bounded binary-safe diagnostics.

The executable has four modules and no service dependency. Direct dependencies are clap, serde, serde_json, sha2, tempfile, and Unix-only nix. The delivered source set includes Cargo.lock. The initial project was committed and pushed as a6fcb97 with the user's authorization.

The v0.1.0 binary distribution targets Linux/WSL x86_64 with a static musl executable. Local packaging retains dependency/runtime notices and build provenance and emits SHA256SUMS. See the [release installation instructions](releases/v0.1.0.md) and [packaging procedure](../CONTRIBUTING.md#release-packaging). Broader platform distribution remains deferred.

## Evidence

See [verification](verification.md) for commands, actual outcomes, onboarding measurements, and platform gaps. Automated tests cover lifecycle, immutability, malformed inputs, stale/corrupt state, nonzero application exits, binary differences, mixed-error precedence, deadlines/descendants, output budgets, environment/cwd, locks, permissions, symlinks, and non-regular files.

The demo catches reversed ordering while the fixture's membership/count checks pass. It repairs the fixture and verifies a pass. This is one controlled example, not a general correctness or customer-value study.

## Next work

M2: read approved contracts from an independently chosen protected Git reference. First define ownership of the reference and the tool executable. Current working-tree hashes cannot provide independent trust against an agent with write access.

Then evaluate real maintainer projects before semantic comparators or adapters. Keep normalization explicit. Do not claim demand or novelty from the documentation survey.

## Known gaps

- Native macOS linking, execution, process cleanup, and performance remain untested. x86_64 macOS cargo check passes.
- No Windows support, hostile process-tree containment, filesystem/network sandbox, or disk/memory/CPU quota.
- No trusted-reference enforcement, signatures, remote policy, discovery, agent integration, filesystem contracts, normalization, or semantic JSON.
- No dependency advisory scan: cargo-audit is unavailable. A metadata/license review and compiler checks are not an advisory audit.
- No interviews, broad benchmark, trademark clearance, package reservation, or telemetry.

Pre-existing .pi state belongs to the goal runtime. Preserve it.
