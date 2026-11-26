# Architecture

One Cargo package and one executable. No daemon, database, service, or mandatory model.

- main: clap parsing, project selection, orchestration, JSON and terminal reports.
- model: strict configuration/snapshot types, semantic validation, hashes, byte comparison, reports.
- storage: bounded no-follow reads, advisory writer locks, same-directory atomic writes, permissions.
- runner: explicit subprocesses, environment/cwd, nonblocking collection, deadlines, group cleanup.

Flow: invar.json → validated Config → execution → complete Snapshot → candidate.json. Explicit identity-pinned lock → accepted.json. Verification preflight → fresh observations → comparison → Report. Verification has no write path.

See [specification](specification.md), [decisions](decisions.md), and [threat model](security/threat-model.md).
