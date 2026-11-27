# Contributing

Use Rust 1.88 or newer. Python 3 and standard Unix commands are required for development fixtures. Runtime users need only their configured commands. Keep Cargo.lock in the delivered file set.

Before editing, read [scope and formats](docs/specification.md), [architecture decisions](docs/decisions.md), and [security boundaries](SECURITY.md).

## Checks

Install `just` with `cargo install just --locked`. Run from the repository root:

~~~sh
just check
cargo build --locked
python3 scripts/demo.py --invar target/debug/invar
python3 scripts/check_first_run.py --invar target/debug/invar
~~~

`just check` runs formatting, Clippy, tests, and minimum Rust version tests. Run `just fmt`, `just lint`, `just test`, or `just msrv` for an individual check.

Install the minimum toolchain with rustup toolchain install 1.88.0 --profile minimal if necessary. Build on Linux and test natively on macOS before claiming runtime support there. A cross-target cargo check does not test linking or process semantics.

Add the smallest test that catches changed nontrivial behavior. Keep errors visible. Do not suppress warnings or delete a failing test to make checks pass. Exercise documented commands in a clean temporary project. Record the actual platform and unavailable checks.

## Release packaging

The initial binary distribution targets Linux x86_64 with musl. Packaging requires Linux x86_64, Python 3, GNU binutils (`strip` and `readelf`), the target standard library, and the compiler's `rust-docs` component for retained library notices:

```sh
rustup target add x86_64-unknown-linux-musl
rustup component add rust-docs
cargo fetch --locked --target x86_64-unknown-linux-musl
python3 scripts/package_release.py
```

The script refuses tracked Rust-source or lockfile changes relative to HEAD. It builds offline with Cargo.lock, strips a copy, checks the version and static ELF linkage, gathers target-graph dependency notices and runtime notices, checks the archive, and writes the archive plus SHA256SUMS under Cargo's target directory in `dist/`. BUILD-INFO.json records the source commit, compiler, target, and binary/lockfile hashes. This does not establish a reproducible or signed build.

The musl COPYRIGHT notice is retained verbatim from upstream commit `7813e9119eb5fe681b6750b8a033a0490bca0941`, retrieved on 2026-11-27: https://git.musl-libc.org/cgit/musl/tree/COPYRIGHT?id=7813e9119eb5fe681b6750b8a033a0490bca0941. This identifies the notice snapshot, not the compiler's bundled musl version. Rust notices come from the toolchain used for the build. The inventory also retains build-time dependency notices.

Run the target tests and demonstrations, inspect the archive, and pin the release to the recorded source commit. Publishing or pushing still requires authorization. Validate uploaded assets before publishing a draft, then download and verify the public release.

### One-command installer

`install.sh` pins the published version and archive SHA-256. It retains the complete release package and updates a user-local executable symlink. It refuses to overwrite a file or foreign symlink at that path. PATH setup preserves existing shell configuration.

After publishing a new archive, update the installer's version and digest from that exact asset. Run the isolated installer checks with a downloaded copy:

```sh
python3 scripts/check_installer.py --archive /absolute/path/to/invar-v0.1.0-x86_64-unknown-linux-musl.tar.gz
```

The checks use temporary homes and local download fixtures. They need Python 3 and standard Linux tools, but make no network requests. They exercise installation, reinstallation, shell configuration, unsupported platforms, collisions, failed downloads, checksum failures, and preservation of an earlier installation. The archive argument must identify the published bytes, not a fresh rebuild.

Commit the installer, attach `install.sh` to the matching GitHub release, and verify the public one-command installation in a temporary home. Do not move an existing release tag, replace the published archive, or rebuild its provenance to include a later installer-only commit.

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
