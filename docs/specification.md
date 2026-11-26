# Invar v0.1 specification

Schema version 1. Product: Agentic Behavioral Verification. Tagline: Change code. Preserve behavior.

## Requirements and boundary

Observe reviewed command examples across languages. Compare exit code, stdout, and stderr exactly. Separate capture, approval, and verification. Provide readable diagnostics and versioned JSON. Empty suites, execution problems, and unknown baselines cannot pass. This complements existing tests. It does not prove general equivalence, discover behavior, judge correctness, or resist a writer who can change the baseline.

## CLI

Run from the project directory or pass --project DIR. No ancestor search or command discovery occurs.

- init creates invar.json and .invar, including a local .gitignore for candidate.json and writer.lock. Refuse existing paths. The example uses printf. Initialization executes nothing.
- capture runs configured cases. Atomically replace .invar/candidate.json only if all observations succeed within the budgets. Never change accepted state. Failed capture leaves the previous candidate intact.
- lock --candidate SHA256 approves the exact candidate file with that full lowercase SHA-256. Inspect its configuration, exits, and hex payloads first. Execute nothing. Require --replace if accepted.json exists. Reject configuration drift or invalid candidate data.
- verify preflights the complete accepted snapshot, runs cases, and reports differences. Never write configuration, candidate, or accepted files.
- --json emits one versioned JSON report and no diagnostic text on stdout, including for usage errors. Help/version requests remain ordinary text.

## Configuration

invar.json is UTF-8 JSON. Reject unknown or duplicate fields, unsupported versions, invalid IDs, NUL strings, and invalid limits. Omitted defaults and explicit defaults have identical meaning.

```json
{
  "version": 1,
  "cases": [{
    "id": "hello",
    "argv": ["printf", "hello\n"],
    "cwd": ".",
    "env": {},
    "timeout_ms": 1000,
    "max_output_bytes": 1048576
  }]
}
```

Case IDs contain ASCII letters, digits, underscores, or hyphens, at most 64 bytes. argv is a nonempty string array. Arguments pass literally without expansion. To opt into a shell, name it explicitly, such as ["sh", "-c", "printf hello"]. This is an explicit command choice, not shell detection or a security control.

cwd is relative to the project root. Reject absolute paths and parent components. Its resolved directory must remain within the canonical project root. Resolve relative executable paths containing a slash against cwd. Bare executable names use PATH.

stdin is closed. Clear the child environment. Supply the invoking process's PATH, or /usr/bin:/bin if absent, plus LANG=C, LC_ALL=C, TZ=UTC. Apply explicit env overrides last. Do not inherit HOME, credentials, or other ambient variables. PATH and executable contents are not fingerprinted. Pin them externally when required. Explicit env values appear in snapshots. Do not put secrets there.

## Execution budgets

| Resource | Limit |
| --- | --- |
| Configuration | 256 KiB |
| Cases | 1–32, sequential execution |
| Per-case deadline | Default 1000 ms, configurable 1–30000 ms |
| Per-stream output | Default/maximum 1 MiB, configurable down to 1 byte |
| Aggregate snapshot output | 8 MiB before encoding |
| Snapshot file read | 20 MiB |
| Stream read batch | At most 64 KiB before checking other stream/deadline |
| Post-exit drain | At most 100 ms, also bounded by case deadline |
| Difference preview | At most 48 bytes from first changed offset |

Both pipes use alternating nonblocking read batches. Give commands their own Unix process group. On timeout, output excess, read error, or ordinary leader exit, terminate remaining same-group descendants. Reap the direct child. Signal exits are execution errors. Normal nonzero application exits are approvable and comparable. Never truncate an observation into a valid candidate.

These limits are not memory, CPU, or disk quotas, a sandbox, or hostile process-tree containment. Commands can change project files and access the network. Escaped sessions need external containment. Kernel-stalled spawn/reap operations are not hard real-time bounded. External termination of Invar can interrupt cleanup.

## Snapshot and approval identity

candidate.json and accepted.json are self-contained JSON artifacts. Each contains:

- version, the semantic config, and config_sha256 of its canonical serde_json representation.
- observations in configured order, with matching IDs and numeric exit codes 0–255.
- stdout/stderr objects, each containing lowercase hex and sha256 of decoded bytes.

Snapshot envelope, observation, and stream fields are required. Embedded configuration uses the same defaults as invar.json. Reject unknown/duplicate fields, malformed hex, digest mismatches, wrong case count/order, excessive bytes, and configuration drift. There are no external artifact references. Missing or corrupt snapshots are baseline failures.

Candidate identity hashes the complete on-disk bytes, including formatting. Lock validates the identity and atomically copies those bytes to accepted.json. Hashes detect corruption. They do not authenticate a reviewer or authorize approval. Anyone with sufficient access can rewrite state and recompute hashes. Protect review externally. Trusted Git references are deferred.

Create owner-only state directories/files. Reject symlink state directories, configuration files, snapshots, and writer locks. Use same-directory temporary files for atomic publication. Capture/lock take a nonblocking advisory writer lock, safely creating its transient file if absent after a clone. Verification uses whole-file reads and writes nothing. Concurrent hostile filesystem replacement is outside the trust boundary. No cross-file transaction or power-loss durability is promised.

## Reports and exits

JSON reports contain version=1, command, status, exit_code, message, candidate_sha256, and cases. Optional values are null. Case records contain id, status, message, changes. Changes name exit_code/stdout/stderr and expected/actual summaries. Stream summaries include lengths, hashes, first differing byte, hex, and escaped previews. Never emit raw control characters in terminal output.

| Exit | Meaning |
| --- | --- |
| 0 | Operation succeeded. Verification matched all cases. |
| 1 | At least one regression, no execution errors. |
| 2 | CLI/configuration/I/O/execution/infrastructure error. |
| 3 | Missing/invalid/mismatched baseline or candidate identity. |

Precedence is phased: CLI/config validation (2), complete baseline preflight (3), then execution (2 overrides 1, then 0). Baseline failure executes no cases. Execution continues after case errors. Capture never publishes a partial suite. No timestamps or elapsed timings enter accepted observations or reports.

## Acceptance

Test the lifecycle, explicit approval identity, candidate isolation, verification immutability, all compared channels, binary bytes, malformed inputs, missing/corrupt baseline, config drift, missing executable, timeout cleanup, simultaneous streams, output limits, environment/cwd, JSON hygiene, and mixed-result precedence. Demonstrate an ordering regression missed by conventional fixture checks, then demonstrate repair.
