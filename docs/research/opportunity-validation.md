# Opportunity validation

Verified: 2026-11-26. Evidence: [landscape](landscape.md), [comparison](competitive-analysis.md), [technical decisions](technical-foundations.md).

## Facts

Established tools already capture, approve, and compare command output. Approval review already exists. Related Invar names already exist. The initial repository contains only a short README, Apache-2.0 license, and Rust ignore rules. This session found no existing executable to extend.

## Product hypothesis

A maintainer may want a standalone local check before and after an agent edits a mixed-language project. Their ordinary tests may omit a user-visible detail, such as ordering or diagnostics. Explicit approval and stable JSON may make this workflow useful across agents. This remains unvalidated demand, not a competitive advantage demonstrated by research.

## Smallest testable workflow

1. Configure reviewed argument arrays in invar.json.
2. Capture byte-exact exit status, stdout, and stderr into a candidate.
3. Inspect that candidate. Approve its exact SHA-256 identity with lock.
4. Edit code. Verify against the accepted snapshot without rewriting it.
5. Read a channel-specific difference. Restore the intended behavior and verify again.

The fixture must demonstrate an ordering regression that its own conventional check misses. This establishes a narrow observable benefit, not superiority to adding a conventional regression assertion.

## Scope recommendation

Deliver init/capture/lock/verify. Keep byte comparison strict. Bound output, cases, snapshot size, and runtime. Separate invalid baselines from infrastructure failures and regressions. Use a single local self-contained snapshot. Defer semantic JSON, normalization, filesystem contracts, discovery, agent adapters, reference-rooted trust, and distribution.

## Unknowns and next experiments

No interviews, retention measurement, ecosystem benchmark, trademark review, or cross-platform execution occurred during research. Measure a scripted quick start, stating that it is an engineering exercise rather than a usability study. After the slice, observe maintainers using it in existing projects. Compare setup effort with Cram or trycmd before claiming differentiation. Prioritize protected baseline references before marketing this as independent protection from an agent with repository write access.

Research stops here because implementation choices have sufficient primary-source support. Remaining uncertainties do not require more infrastructure or a broader MVP.
