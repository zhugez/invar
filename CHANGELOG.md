# Changelog

## 0.1.0 — 2026-11-27

- Add local init, capture, explicit hash-pinned lock, and read-only verify commands.
- Compare numeric application exits and exact stdout/stderr bytes, including binary data.
- Separate candidates from accepted self-contained snapshots. Validate configuration and artifact digests before verification.
- Add bounded Unix stream collection, deadlines, output budgets, ordinary process-group cleanup, advisory writer locks, and atomic publication.
- Add versioned JSON reports, channel-specific byte differences, and documented exit codes 0/1/2/3.
- Add lifecycle and failure tests, an ordering-regression demonstration, primary-source research, specifications, and security documentation.
- Add a prebuilt Linux/WSL x86_64 static musl archive, SHA256SUMS, retained third-party notices, and build provenance for GitHub Releases.
- Add a checked local packaging script and release installation instructions.

crates.io publication remains disabled. Local baselines remain mutable by writers with sufficient access. Native macOS execution remains unverified. This release has no macOS or Windows binary.
