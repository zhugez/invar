# Landscape

Verified: 2026-11-26. Scope: primary documentation, not independent product benchmarks.

| Category | Primary source | Observed capability | Implication |
| --- | --- | --- | --- |
| CLI regression | [Cram](https://github.com/brodie/cram) | Runs shell transcripts and compares output. Offers interactive approval and environment normalization. | Language-independent command regression is established. |
| CLI snapshots | [trycmd](https://docs.rs/trycmd/latest/trycmd/) | Rust harness with command fixtures, stdout/stderr, binary support, filesystem checks, and overwrite mode. | Exact output comparisons and fixture capture are not novel. |
| Approval testing | [ApprovalTests](https://approvaltests.com/) | Capture results, review them, approve them, then detect changes. | Explicit approval has a long precedent. |
| Snapshot review | [Insta CLI](https://insta.rs/docs/cli/) | Separate review, accept, reject, and pending-snapshot operations. | Candidate/accepted separation should be a minimum expectation. |
| Service contracts | [Pact](https://pact.io/) | Consumer/provider integration contracts and compatibility workflows. | A command observer complements service contracts, rather than replacing them. |
| Generated API tests | [Schemathesis](https://schemathesis.readthedocs.io/) | Generates property-based API tests from service specifications. | Observed command examples do not provide generated-input coverage. |
| Similarly named verification | [Ramesh-35/invar](https://github.com/Ramesh-35/invar) | Its README describes cross-repository field contracts, lockfiles, local checks, receipts, and agent integration. | Both naming and positioning overlap. Its claims were read, not independently reproduced. |

## Method and limits

Read the linked project documentation with agent-browser. Public Google search was unavailable because its authentication was unavailable. Direct primary documentation remained accessible. A mistaken rust-cli/trycmd URL returned 404. The docs.rs primary documentation was accessible instead.

No stars, download counts, growth claims, customer interviews, or competitor benchmarks support this assessment. Features absent from this table are unassessed, not proven absent. Current pages can change after this verification date.
