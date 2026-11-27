<p align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="brand/assets/logo-reversed.svg">
    <source media="(prefers-color-scheme: light)" srcset="brand/assets/logo-primary.svg">
    <img src="brand/assets/logo-primary.svg" alt="Invar" width="272" height="66">
  </picture>
</p>

<h1 align="center">Change code. Preserve behavior.</h1>

<p align="center">A local behavior check for code changes.</p>

<p align="center">
  <a href="#install">Install</a> ·
  <a href="#quick-start">Quick start</a> ·
  <a href="#see-it-catch-a-regression">Demos</a> ·
  <a href="#documentation">Documentation</a>
</p>

Capture reviewed commands, approve a baseline, then compare later runs against it. Invar checks **exit code, stdout, and stderr byte for byte**, alongside your existing tests.

## Install

```sh
curl -fsSL https://github.com/zhugez/invar/raw/main/install.sh | sh
```

**Linux / WSL x86_64 · v0.1.0 · No sudo or Rust required.**

The installer checks a pinned SHA-256 and sets up your shell's PATH when supported. Open a new terminal afterward. Your configured commands still need their own runtimes.

[Manual installation](docs/releases/v0.1.0.md) · [Build from source](docs/getting-started.md#build-from-source)

## Quick start

Run these commands from your project's directory.

1. **Choose what to preserve.**

   ```sh
   invar init
   ```

   Review `invar.json`. Replace the sample `printf` case with a command you want to check. Initialization executes nothing. See the [configuration format](docs/specification.md#configuration).

2. **Capture a candidate.**

   ```sh
   invar capture
   ```

   This records command results in `.invar/candidate.json`. It does not approve them.

3. **Review and approve.**

   Inspect the captured configuration, exit code, stdout, and stderr. Set `REVIEWED_SHA256` to the exact hash of the file you reviewed. The [review recipe](docs/getting-started.md#use-your-own-project) shows how to decode captured bytes and hash that same file.

   ```sh
   invar lock --candidate "$REVIEWED_SHA256"
   ```

4. **Change code, then verify.**

   ```sh
   invar verify
   ```

   A changed result reports a regression. Verification never rewrites the configuration, candidate, or accepted baseline. Configured commands can still write files.

Commit `invar.json`, `.invar/.gitignore`, and `.invar/accepted.json` for review. Do not automatically recapture and approve after each code change.

## See it catch a regression

The ordering demo captures sorted names, reverses their order, then restores it: **pass → regression → pass**. The regression report begins:

```text
verify: regression (exit 1)
  sorted-names: regression
    stdout changed
```

Run it from a source checkout with Python 3 and Invar on PATH:

```sh
python3 scripts/demo.py --invar "$(command -v invar)"
```

The fixture's membership/count checks stay green because they omit ordering. An ordering assertion would also catch the change.

The [CLI self-check](docs/verification.md#invar-checks-its-own-json-interface) injects a JSON compatibility regression into an isolated copy of Invar itself, detects it, and verifies the repair.

## Results and boundaries

Use `invar --json verify` for a versioned, machine-readable report.

| Exit | Meaning |
| --- | --- |
| `0` | Success. Verification matched all configured examples. |
| `1` | Behavior regression. |
| `2` | Configuration, execution, or infrastructure error. |
| `3` | Missing, invalid, or mismatched baseline. |

- **Examples, not a proof.** A pass covers the commands you configured. Comparison is exact, with bounded runtime and output. There is no normalization or semantic JSON comparison yet.
- **Approval needs review.** Hashes pin a candidate. They do not authenticate a reviewer. Protect the executable, configuration, and baseline from unauthorized changes.
- **Commands are not sandboxed.** They run with your filesystem and network access. Review commands before capture. Read the [security policy](SECURITY.md).

## Documentation

- [Getting started](docs/getting-started.md) — detailed installation, source builds, demonstrations, and baseline review.
- [Specification](docs/specification.md) · [Architecture](docs/architecture.md) — formats, execution rules, limits, and storage.
- [Contributing](CONTRIBUTING.md) · [Verification evidence](docs/verification.md) — development checks and what has been exercised.
- [Roadmap](ROADMAP.md) · [Engineering state](docs/engineering-state.md) · [Research](docs/research/landscape.md) — current scope, gaps, and decisions.
- [Brand kit](brand/README.md) — proposed identity, logo assets, colors, and typography.

[Apache-2.0](LICENSE)
