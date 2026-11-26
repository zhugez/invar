# Architecture decisions

Accepted 2026-11-26 before implementation.

1. **Standalone command boundary.** Reuse std::process and reviewed argv arrays. Language support follows available commands. Defer discovery and adapters.
2. **Strict byte equivalence.** Compare three channels exactly. Hex preserves binary bytes. Defer normalization and semantic comparisons until real use justifies them.
3. **Explicit approval.** Capture cannot accept. Lock requires the candidate's complete hash. Replacement needs a flag. Verification has no write mode.
4. **Self-contained snapshots.** Two bounded JSON files replace a manifest/blob store. Atomic publication avoids cross-file inconsistency. Hex costs space and needs inspection tools.
5. **Local trust.** Checksums detect corruption, not coordinated tampering. Protected-reference loading is the next trust milestone.
6. **Unix without an async runtime.** std process groups and safe nix wrappers support bounded alternating reads and ordinary descendant cleanup. Defer Windows instead of pretending cleanup works.
7. **Reviewed dependencies.** Serde/JSON, sha2, nix, tempfile, clap. No homegrown crypto, temporary filenames, or argument parser. Forbid first-party unsafe. Lock dependencies. Rust minimum 1.88 requires actual verification.
8. **No distribution.** Use package invar-behavior, binary invar, publish=false. Naming conflicts require a separate decision. Preserve Apache-2.0.
