# First-user launch readiness

Verdict: **prepared locally for a small first-user recruitment experiment**, subject to publication and the live checks below. This is not a claim of market demand, paid readiness, general correctness, or universal platform support.

## What is ready

- [Launch strategy and copy](launch.md): one audience hypothesis (CLI/script maintainers using coding agents), stable-output use case, primary-source alternatives, interview questions, manual tracking, and experiment decision rules. Drafts are ready for owner review; none were sent.
- [README](../README.md) and [landing-page source](../site/public/index.html): aligned offer, primary try-it action, installation scope, real walkthrough links, feedback, and explicit limitations. The existing visual identity remains intact.
- [Standalone first run](first-run.md): installed executable, disposable project, inspected candidate, explicit hash approval, regression, repair, and cleanup. No checkout is needed for the user exercise.
- [Pi setup guide](../integrations/pi/README.md): load the exact extension, select a command, review configuration, confirm capture, inspect candidate, separately approve, verify, and disable. Existing configuration bypasses the suggestion picker.
- [First-use issue form](../.github/ISSUE_TEMPLATE/first-use.yml): platform/version, use case, stopping point, expected/observed result, optional synthetic reproduction and alternatives, required privacy acknowledgement. Blank issues remain available; security reports use the private channel.

## Observed verification

Environment observed in this session: Linux x86_64, Python 3.12.3, Node.js v22.23.5, installed Invar 0.1.0. Checks used isolated temporary homes/projects and did not approve a baseline for this repository.

| Check | Observed result |
| --- | --- |
| GitHub release metadata | v0.1.0 includes installer, Linux musl archive, and SHA256SUMS. |
| Current public installer identity | GitHub `contents/install.sh` blob SHA matched local `git hash-object install.sh`. |
| Real network installation | Ran unchanged `install.sh` through `/bin/sh` in an isolated HOME with a minimal environment. Download, pinned checksum, executable version, installation and PATH instruction succeeded. This checked the installer/release path, not a browser-run shell pipeline. |
| `python3 scripts/check_installer.py --archive <downloaded release archive>` | 12 checks passed, including repeat install, spaces, notices, PATH, unsupported OS/architecture, collisions, checksum/download failure, and truncated script. Temporary homes removed. |
| `python3 scripts/check_first_run.py --invar <freshly installed binary>` | Executed the actual first-run shell blocks after installation. Reviewed capture, pass → regression → pass, expected regression exit 1, and disposal all passed. |
| `python3 scripts/demo.py --invar <freshly installed binary>` | Original ordering demo passed through regression and repair; candidate and accepted bytes remained unchanged during verification. |
| `node integrations/pi/check.mjs <installed Pi package directory> <freshly installed binary>` | 17 checks passed: real extension loading/executable with mocked interactive UI, command suggestions, declined capture executing nothing, separate approval, drift/trust boundaries, bounded repair, and disabling. |
| `just check` | Exit 0: formatting, warnings-denied Clippy, stable tests, Rust 1.88.0 tests. No aggregate test-count claim is made. |
| `node site/check.mjs http://127.0.0.1:8793` | Passed at 1440, 768, 390, 320 pixels: no horizontal overflow, both demo results, copied installer, 9 assets, local anchors, correct first-run/source URLs and feedback presence. Primary CTA reached installation at desktop and mobile widths. |
| Keyboard skip link | Enter moved focus to the main heading (`hero-title`). |
| Screenshot inspection | Desktop and phone full-page renders inspected; layout and added copy fit. The final copy refinement retained the same layout and passed browser checks. |
| Impeccable detector | Final regex pass reported no findings. HTML parser modules were unavailable; computed contrast/selectors were not audited. This is not a full accessibility certification. |
| Issue-form/config YAML | Parsed successfully with available PyYAML; IDs/types, required fields/privacy checkbox, blank fallback, and private-security link checked. No dependency added. |
| Existing feedback infrastructure | Repository API reports Issues enabled; private vulnerability reporting enabled. Opening the feedback URL through agent-browser reached GitHub sign-in. No issue was submitted. |
| Link/whitespace/syntax checks | Changed local Markdown targets and repository source links checked against local files/headings; JS syntax and `git diff --check` passed. New public guide/form URLs require publication before their live rendering can be checked. |

The site test initially clicked Copy during resize/anchor scrolling and failed to activate it. The test now checks copying before separately exercising navigation at desktop/mobile sizes; both paths pass. No failing assertion was removed or suppressed.

## Limits and unresolved hypotheses

- No external usability sessions, activated users, retention, willingness-to-pay evidence, or paid acquisition measurements were collected. The initial audience and channels are hypotheses. Proposed trial counts/time windows in the runbook are experiment choices.
- Installation/runtime was checked on this Linux x86_64 environment. Native macOS linking/execution and native Windows support are not established; the installer is Linux/WSL x86_64 only.
- Pi was checked with its installed extension loader and mocked UI, not a human-driven terminal trial. A first external Pi user should be observed following the guide unaided.
- GitHub requires sign-in to submit feedback. The new form has not been rendered on GitHub because it is not published yet. An ordinary public issue is the existing fallback.
- Browser checks used the local updated site. The live site still needs deployment and a repeat smoke check. No screen-reader or full cross-browser certification is claimed.
- Exact byte comparison flags nondeterminism. It covers configured examples and does not sandbox commands. Hashes do not authenticate reviewers. Competitor capabilities overlap; no novelty or superiority claim is justified.
- Primary-source competitor pages were inspected directly. Google research tooling lacked credentials; no broad market survey was completed or needed to claim this bounded preparation.

## Owner actions, in order

1. Review the changes and authorize a commit/push. The previous “push all” was completed before this goal; it does not publish these new changes.
2. After publication, open the README, `docs/first-run.md`, Pi guide, and `issues/new?template=first-use.yml` on GitHub. Confirm the form renders and its security/blank-issue routes work. Do not submit test feedback publicly without intent.
3. Authorize deployment of the updated landing page. Repeat the browser check against its public URL and follow the first-run link. Verify the GitHub source guide and installation route before sharing posts.
4. Use the [founder runbook](launch.md#founder-runbook) to invite the initial small group and observe trials. Publication/outreach requires owner action; nothing was sent on your behalf.
5. Review stopping points and real-command reuse before building more features or designing a paid product. Record evidence that argues against the hypothesis as well as evidence that supports it.

No release, deployment, commit, push, public post, outreach message, billing system, tracking service, or support SLA was created by this preparation.
