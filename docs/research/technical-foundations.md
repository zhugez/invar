# Technical foundations

Verified: 2026-11-26. Primary documentation and cargo info informed these choices.

## Execution

[std::process::Command](https://doc.rust-lang.org/std/process/struct.Command.html) accepts individual arguments and supports environment clearing and piped streams. Avoid output(), which collects unbounded output. Use a Unix process group through [CommandExt::process_group](https://doc.rust-lang.org/std/os/unix/process/trait.CommandExt.html). Use safe nix wrappers for nonblocking pipes and group termination. Read both streams in bounded alternating batches under a wall-clock deadline. This provides concurrent collection without an async runtime or reader threads that can remain blocked.

Process groups cannot contain a hostile process that creates a new session. Deadlines are not memory or CPU quotas. A temporary directory would not provide security isolation. Run reviewed code with ordinary user privileges or use an external sandbox.

## Serialization and storage

Use JSON for configuration, snapshots, and reports. Strict Serde structures reject unknown fields. BTreeMap fixes environment-key ordering. Raw bytes use lowercase hexadecimal with SHA-256 checks. A self-contained snapshot avoids cross-file publication inconsistencies and path references. Atomic replacement uses a same-directory temporary file followed by rename. Advisory writer locking prevents cooperating capture/approval operations from racing. Verification reads only.

SHA-256 detects accidental payload corruption. It does not authenticate an author, authorize approval, or stop an agent rewriting both payload and digest.

## Direct dependency review

Registry metadata checked with cargo info. Cargo.lock records actual transitive resolution. These are adoption decisions, not a security audit.

| Crate | Selected version family | Reported license | Reported Rust minimum | Reason |
| --- | --- | --- | --- | --- |
| serde | 1.0.229 | MIT OR Apache-2.0 | 1.56 | Strict typed structures and derive. |
| serde_json | 1.0.154 | MIT OR Apache-2.0 | 1.71 | Shared JSON format. |
| sha2 | 0.11.2 | MIT OR Apache-2.0 | 1.88 | Established hashing implementation. |
| nix | 0.31.4 | MIT | 1.69 | Safe Unix descriptor/signal/flock APIs. Disable default features. |
| tempfile | 3.27.0 | MIT OR Apache-2.0 | 1.63 | Secure temporary files and atomic persistence. |
| clap | 4.6.10 | MIT OR Apache-2.0 | 1.85 | Validated CLI, help, argument errors. Disable color/suggestions. |

Primary crate sources: [serde](https://github.com/serde-rs/serde), [serde_json](https://github.com/serde-rs/json), [sha2](https://docs.rs/sha2/0.11.2/sha2/), [nix](https://github.com/nix-rust/nix), [tempfile](https://github.com/Stebalien/tempfile), [clap](https://github.com/clap-rs/clap).

Set the project MSRV to Rust 1.88, then test the locked graph on that toolchain. Forbid unsafe code in first-party Rust. Dependencies and standard-library internals contain unsafe code. Do not claim otherwise. Review the transitive license set from cargo metadata. Check advisories when tooling permits. [RustSec](https://github.com/RustSec/advisory-db) describes cargo-audit as a lockfile advisory checker. A successful compiler run alone is not an advisory check.

The current docs.rs build for nix 0.31.4 failed. Review the downloaded source and compile/test the used APIs rather than assuming generated documentation exists.

Implementation follow-through: the locked graph passes tests on Rust 1.88.0. Resolved metadata reports (MIT OR Apache-2.0) AND Unicode-3.0 for unicode-ident, MIT-only packages, and packages offering MIT as an alternative license. Review the actual dependency notices before binary distribution. cargo-audit was not installed, so no advisory scan occurred. See the [recorded checks](../verification.md).
