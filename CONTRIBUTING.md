# Contributing

Use Rust 1.88 or newer. Python 3 and standard Unix commands are required for development fixtures. Runtime users need only their configured commands. Keep Cargo.lock in the delivered file set.

Before editing, read [scope and formats](docs/specification.md), [architecture decisions](docs/decisions.md), and [security boundaries](SECURITY.md).

## Checks

Run from the repository root:

~~~sh
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked --all-targets
cargo +1.88.0 test --locked --all-targets
cargo build --locked
python3 scripts/demo.py --invar target/debug/invar
~~~

Install the minimum toolchain with rustup toolchain install 1.88.0 --profile minimal if necessary. Build on Linux and test natively on macOS before claiming runtime support there. A cross-target cargo check does not test linking or process semantics.

Add the smallest test that catches changed nontrivial behavior. Keep errors visible. Do not suppress warnings or delete a failing test to make checks pass. Exercise documented commands in a clean temporary project. Record the actual platform and unavailable checks.

## Engineering rules

- Preserve candidate/accepted separation. Capture cannot approve. Verification cannot write expectations.
- Treat configuration, snapshot, CLI, and report changes as interface changes. Update their version or preserve compatibility deliberately.
- Keep commands explicit. Do not execute discovered commands or introduce an implicit shell.
- Preserve bounded reads, deadlines, output caps, and cleanup on every error path.
- First-party Rust forbids unsafe code. This does not imply dependencies or standard-library internals contain no unsafe code.
- Prefer existing modules, the standard library, and installed dependencies. Research a new dependency's primary documentation, license, and minimum Rust version first.
- Keep research facts separate from product hypotheses. Do not invent benchmarks, platform checks, or security claims.
- Do not capture secrets. Do not publish, deploy, push, or create commits without separate authorization.

Apache-2.0 applies to the project. Preserve the existing LICENSE. Review dependency notices before distributing binaries. See the [roadmap](ROADMAP.md) for deferred work.
