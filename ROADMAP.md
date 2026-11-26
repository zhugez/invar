# Roadmap

| Milestone | Scope | Exit criterion |
| --- | --- | --- |
| M0 | Research, naming, requirements, architecture, threat model | Source-backed decisions and explicit limits. |
| M1 | init/capture/lock/verify, three channels, bounded execution, JSON | Regression/repair demonstration and failure-case tests. |
| M2 | Protected Git-reference baselines and CI trust policy | Changed working-tree baselines cannot choose their judge. |
| M3 | Semantic JSON and scoped normalization | Opt-in rules with tests against hidden regressions. |
| M4 | Filesystem contracts | Bounded paths, bytes, and side-effect semantics. |
| M5 | Agent adapters and reviewed discovery proposals | Agent-independent core and no automatic untrusted execution. |
| M6 | Real-project evaluation | Recorded maintainer trials and competitor/setup comparisons. |
| M7 | Distribution and platforms | Resolved naming, packaging, provenance, native checks. |

M0/M1 are the current goal. Later rows are backlog, not implemented interfaces. No cloud dashboard, LLM judge, telemetry, or mandatory service is needed for this slice.
