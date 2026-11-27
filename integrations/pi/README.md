# Invar with Pi

This adapter checks reviewed commands after potentially modifying Pi tools. It is an optional source-checkout integration, not part of the v0.1.0 binary archive.

## Load the adapter

Install Invar using the [installation guide](../../docs/getting-started.md#install-from-github-releases). Install Pi separately. From a trusted checkout of this repository, record the extension's absolute path:

```sh
INVAR_EXTENSION="$PWD/integrations/pi/invar.ts"
```

Change into the project you want to protect. Then launch Pi with that path:

```sh
pi -e "$INVAR_EXTENSION"
```

The adapter defaults to `~/.local/bin/invar`. If your executable lives elsewhere, set `INVAR_BIN` to its absolute path before launching Pi. Review the extension source before loading it. Trust the project in Pi before setup.

## Choose and approve a command

1. Run `/invar setup` in Pi's interactive UI.
2. For a new configuration, choose a detected test/check command or **Enter a command manually**. Manual entry uses a JSON argument array, for example `["python3", "report.py"]`. Detection reads project files and executes nothing. It recognizes package metadata/lockfiles for npm, pnpm, bun, and yarn, Cargo, and common Justfile recipes. It does not parse every just syntax or supply required recipe arguments.
3. Review the configuration. **Save configuration and capture?** runs commands only if you confirm. Declining this confirmation runs nothing and does not save a new configuration.
4. Inspect the captured command arguments, exit codes, and escaped stdout/stderr bytes. Save the preview unchanged to continue. Escape or any edit cancels approval.
5. Confirm **Approve this exact candidate?**, or the replacement prompt if a baseline exists, only if you accept the reviewed behavior. Setup reruns verification before enabling automatic checks.

If `invar.json` already exists, setup opens it for review and skips the suggestion picker. Do not remove an existing baseline merely to see the new picker. If a different extension copy is already installed, load the file above explicitly; editing this checkout does not update other copies. `/reload` reloads auto-discovered extensions in Pi's extension directories.

Configured commands have your filesystem/network permissions. A confirmed capture is not baseline approval. Hashes identify bytes, not trusted reviewers. Keep command output free of secrets.

## Daily use and recovery

- `/invar status`: show whether this project is enabled.
- `/invar verify`: verify now. Pi also checks before finishing after potentially modifying tools.
- `/invar off`: disable automatic verification while preserving accepted files.
- A regression requests a bounded repair. Repair code rather than changing the baseline to pass.
- Changed configuration, baseline, or executable fingerprints stop verification. Restore the reviewed inputs or deliberately review again with `/invar setup`.
- Oversized candidates cannot be reviewed inline. Review through the [CLI workflow](../../docs/getting-started.md#use-your-own-project).

This integration was exercised through Pi's extension loader and mocked interactive UI with the real Invar executable. That check covers the setup selection/approval lifecycle and declined execution. It does not replace a human usability trial in the Pi terminal UI.

## Adapter self-check

With Node.js, the installed Pi package directory, and Invar available:

```sh
node integrations/pi/check.mjs /absolute/path/to/pi-coding-agent "$(command -v invar)"
```

The check creates temporary projects, loads the real adapter, and cleans its fixtures. It does not register your actual project or approve its baseline. See [first-use feedback](https://github.com/zhugez/invar/issues/new?template=first-use.yml) if setup is confusing.
