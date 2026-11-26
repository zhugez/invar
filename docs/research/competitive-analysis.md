# Competitive analysis

Verified: 2026-11-26. Source facts below come from project documentation. They are not results of executing competitors.

| Tool | Documented focus | Invar v0.1 decision |
| --- | --- | --- |
| [Cram](https://github.com/brodie/cram) | Language-independent shell transcripts, escaped binary output, interactive merging. | Direct argument arrays by default. No implicit shell. |
| [trycmd](https://docs.rs/trycmd/latest/trycmd/) | CLI snapshots with separate streams, binary and filesystem support. | Do not claim better coverage. Use a standalone executable without a Rust test harness. |
| [ApprovalTests](https://approvaltests.com/) | Human-approved captured results. | Capture and approval must remain separate operations. |
| [Insta](https://insta.rs/docs/cli/) | Pending snapshots and explicit review/accept/reject. | Approval is established practice, not a new category. |
| [Pact](https://pact.io/) | Consumer/provider compatibility. | Do not build service virtualization or a broker. |
| [Schemathesis](https://schemathesis.readthedocs.io/) | Property-based API testing. | No claim to input generation or correctness beyond observed examples. |
| [Ramesh-35/invar](https://github.com/Ramesh-35/invar) | Cross-repository contracts, schema judging, baseline references, agent integration. | Record substantial brand/positioning overlap. Do not imitate unverified product claims. |

## Naming check

Ran cargo info on 2026-11-26. The registry contains invar 0.6.0, but its manifest uses a nightly Cargo feature, so cargo info could not finish. It also contains [invar-cli](https://crates.io/crates/invar-cli) 1.0.0, described as generating structural cache keys. [invar-falsify](https://github.com/Ramesh-35/invar) uses the Invar name for another verification tool.

The query for invar-behavior returned not found. Use that package name locally, with binary name invar and publishing disabled. This is not a reservation or trademark clearance. Recheck identity, executable collisions, registry availability, and legal suitability before distribution. Keep the user's repository name and tagline.

## Bounded conclusion

The evidence rejects a novelty claim for output comparison, approval, command replay, or agent compatibility. The proposed niche is an approval-first standalone command boundary with strict candidate separation, machine-readable error classes, bounded collection, and clear trust limitations. This combination is a product hypothesis. No claim establishes that competitors cannot offer it.
