# Threat model

Assets: accepted behavior, candidate identity, honest verdicts, project files, resources, and credentials. Actors: maintainer, coding agent, configured command, CI contributor. Trust the installed Invar executable and dependencies. Review configuration before execution.

| Abuse or boundary | Mitigation | Remaining limit |
| --- | --- | --- |
| Capture silently accepts changes | Separate candidate/accepted files and hash-pinned lock | A writer can still edit accepted.json or invoke lock. |
| Corrupt/stale snapshot | Strict schema, bounded reads, config and byte digests before execution | Recomputed hashes do not authenticate authors. |
| Shell injection | Literal arguments, no implicit shell or command discovery | Explicit shells/interpreters execute arbitrary code. |
| Secret inheritance | Clear environment and use a small allowlist | Commands can read disk/network secrets. Explicit env/output enter snapshots. |
| Pipe deadlock/output excess | Alternating nonblocking reads, caps, deadlines, group cleanup | Escaped sessions and kernel stalls need external containment. |
| Artifact path manipulation | Fixed filenames, no external references, no-follow reads | Concurrent hostile directory replacement is outside local trust. |
| Concurrent approval/capture | Nonblocking advisory flock and atomic rename | External writers can ignore locks. |
| Agent changes tool and baseline | Protect executable, config, baseline, and CI externally | v0.1 has no independent trust root. |
| Command filesystem/network side effects | Explicit reviewed execution as ordinary user | No sandbox, rollback, memory/CPU/disk quota, or network isolation. |

A temporary directory is not a security sandbox. Use independently configured isolation for untrusted code. Do not run privileged CI against untrusted branches. External termination of Invar may interrupt cleanup. Process groups are not hostile process-tree containment.

Next: load approved contracts from an independently selected protected Git reference. Define ownership before signatures or receipts. A checksum without a protected trust root cannot authorize approval.
