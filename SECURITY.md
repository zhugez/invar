# Security

Invar v0.1 uses locally trusted configuration and snapshots. It is not an independent judge when an agent can modify the executable, configuration, accepted snapshot, or CI policy.

- Hashes detect corruption and identify a reviewed candidate. They do not authenticate a reviewer or authorize approval.
- Capture and verification execute configured commands as your user. Review every command first. Invar discovers no commands and makes no network requests itself.
- Commands can read and modify files, spawn processes, access networks, and print secrets. Explicit environment values and captured output enter snapshots. Do not capture credentials.
- The child environment clears ambient variables except PATH and fixed locale/timezone defaults. This does not prevent a command reading secrets elsewhere.
- Deadlines, byte limits, and process groups bound ordinary collection and cleanup. They do not provide a security sandbox or resource quotas.
- Processes that escape their group require an external supervisor or isolation boundary. External termination of Invar can interrupt cleanup.
- Rejecting symlinks and using atomic writes does not prevent a hostile concurrent filesystem writer. Advisory locks coordinate cooperating writers only.
- Verification does not rewrite expectations. The commands it runs can still change the repository.

Protect baselines through ordinary source review and external access controls. Run untrusted code only inside independently configured isolation. Never give untrusted branch code privileged credentials.

See the [threat model](docs/security/threat-model.md) and [specification](docs/specification.md). Protected Git-reference baseline enforcement is the next trust milestone.

## Reporting

Report vulnerabilities through [GitHub private vulnerability reporting](https://github.com/zhugez/invar/security/advisories/new). This channel is enabled for the repository and requires a GitHub account. Do not post credentials or exploit-sensitive details publicly.

For non-sensitive defects, provide the version, platform, minimal configuration, expected behavior, and observed result. Never include real secrets in captured artifacts.
