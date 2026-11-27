# First-user launch

Status: preparation for a small external-user experiment. This is not evidence of demand or product-market fit. Publication and outreach remain owner actions.

## Audience and offer

**Initial audience hypothesis:** maintainers of small CLI tools and repository scripts, using coding agents on Linux/WSL x86_64, with a deterministic command whose output matters to a consumer.

**Job:** before an agent-assisted refactor, preserve a reviewed example of exit code, stdout, and stderr. Afterward, see whether that example changed without adopting a language-specific test harness.

**Offer:** “Change code. Preserve behavior. A local command-output check for maintainers using coding agents.” Start with one command, keep existing tests, and review changes explicitly.

Good first cases include sorted reports, generated text without timestamps, and stable CLI JSON. Poor first cases include live network responses, random IDs, timestamps, secrets, and interactive programs. Invar compares bytes exactly and does not normalize them.

## Evidence and alternatives

Public documentation inspected through agent-browser during this preparation:

| Source | Observed capability | Implication for positioning |
| --- | --- | --- |
| [trycmd](https://docs.rs/trycmd/latest/trycmd/) | Enumerates CLI cases in a Rust test harness; supports stdout/stderr snapshots and documentation examples. Its tool-selection section also recommends language-independent Cram. | Output snapshots and language independence are established options. Do not claim Invar invented them. |
| [Insta CLI](https://insta.rs/docs/cli/) | Reviews pending snapshots with accept/reject/skip; has bulk accept commands. | Human review is established practice. Explain Invar's separate capture and hash-pinned approval without claiming unique safety. |
| [snapbox](https://github.com/assert-rs/snapbox) | Snapshot toolbox for return values, CLI streams, and filesystem changes. The former trycmd repository URL redirects here. | Do not claim broader coverage. Invar's current boundary is process exit/stdout/stderr. |

Repository evidence: [specification](specification.md), [security boundaries](../SECURITY.md), [ordering demonstration](../scripts/demo.py), and [prior competitive/naming review](research/competitive-analysis.md). These establish product behavior, not customer demand. The public search service was unavailable in this session; direct primary documentation supplied the bounded research instead.

The first-segment choice is a hypothesis based on fit with those implemented boundaries. No interviews, external customer cases, willingness to pay, or channel conversion data are claimed. Naming overlaps remain recorded in the prior review; always identify this project as `zhugez/invar` when sharing it.

## Adoption blockers and response

In priority order, based on inspection of the existing newcomer path:

1. The real demo required a source checkout; add a [standalone first run](first-run.md) with a disposable project and inspected candidate.
2. Broad “behavioral guardrail” language leaves the concrete use case unclear; align README and landing-page copy around stable command output and coding-agent refactors.
3. Landing-page workflow/source links pointed to absent README anchors; point to the actual detailed guides.
4. No structured first-use feedback route existed; use a GitHub issue template and an ordinary issue link. This needs no hosted backend.
5. Installer support is Linux/WSL x86_64; native macOS runtime remains unverified. State that restriction rather than promising universal installation.

## Founder runbook

Treat the following quantities as chosen experiment targets, not forecasts or adoption claims. Run one initial round: invite up to **5** relevant maintainers, aim to observe **3** complete trials, and review the results after **7 days**. If recruiting takes longer, record the shortfall instead of silently counting invitations as trials.

1. Publish the reviewed repository changes and deploy the reviewed landing page only after owner approval. Confirm public guide links and the issue template after publication.
2. Start with maintainers you already know who have stable CLI output and use coding agents. Ask permission to observe a trial. Do not post on their issue trackers as advertising.
3. Share one public build/demo post in an existing developer community where self-promotion is permitted. A Show HN submission or a relevant CLI/agent community is a channel hypothesis, not a guaranteed acquisition source. Read that community's current rules before posting.
4. Let each person follow the README unaided first. Record where they stop. Offer help afterward and mark the trial as assisted.
5. Ask them to choose one harmless real command. A completed toy demo is first value; a reviewed baseline plus verification after a real edit is activation. Ask later whether they reused it on another change.
6. Review the evidence before adding features or discussing a paid offer.

### Interview prompts

- What command output changed unexpectedly during your last refactor? What consumed it?
- How do you catch that change today? Show the existing check if you can share it safely.
- Where did installation or baseline review become unclear?
- Which harmless, deterministic command would you preserve in your own repository?
- Did the diff help you decide whether to restore code or deliberately approve a changed contract?
- Would you run this on another change? What would make you use your current snapshot tool instead?

Do not ask for secrets, customer data, private repository access, or raw accepted/candidate snapshots. Use a synthetic reproduction.

### Manual tracking

Keep a private local spreadsheet or note. Do not commit participant identities or private feedback. No product analytics are required for this round.

| Field | Meaning |
| --- | --- |
| Participant alias / source | Anonymous alias and referral/channel; permission to follow up |
| Fit / platform | Stable command, coding-agent use, supported platform |
| Started / installed | Observed attempt and version command success; count separately |
| Demo / real activation | Toy pass → regression → pass versus a real reviewed baseline and later verification |
| Assisted / stopping point | Help given and the exact confusing step or sanitized error |
| Time / repeat use | Measured elapsed time if observed, and later reuse if confirmed |
| Existing alternative / next action | Tool currently used, concrete reason to switch or stay, owner and next step |

Decision rules for this initial round:

- A reproducible install or review failure blocks broader promotion until fixed and rechecked.
- Repeated confusion at the same step warrants one focused onboarding fix before another round.
- If trials finish but nobody chooses a real command, revisit the use case and audience before adding features.
- If users already solve the job comfortably with Cram, trycmd, Insta, or existing tests, record that outcome. Do not manufacture differentiation.
- If users activate and independently reuse Invar, ask what workflow they would miss without it. That is evidence for further discovery, not proof of paid demand.
- If invitations yield no trials, test relevance and distribution first. Silence does not isolate a product defect.

## Launch copy — drafts, not sent

### Short social/community post

I built Invar (`zhugez/invar`) for maintainers who use coding agents and want a reviewed reference for stable CLI output. Capture a command, inspect its exit code/stdout/stderr, explicitly approve the candidate, then compare later runs byte for byte.

It's local and Apache-2.0. The prebuilt installer supports Linux/WSL x86_64. It complements tests; it doesn't prove correctness or sandbox commands.

Try the standalone pass → regression → pass walkthrough: https://github.com/zhugez/invar/blob/main/docs/first-run.md

I'm looking for first-use feedback: where did you get stuck, and which real command would you preserve? https://github.com/zhugez/invar/issues/new?template=first-use.yml

### Show HN draft

Title: Show HN: Invar — reviewed command-output baselines for code changes

I wanted a standalone way to keep a known command result as a reference while changing code with a coding agent. Invar captures exit code, stdout, and stderr, separates candidate capture from explicit approval, and compares later runs against accepted examples.

Snapshot and approval testing already exist. Invar explores a small standalone CLI workflow with direct argument arrays and a Pi integration. It doesn't replace tests, normalize output, or sandbox commands. Approval hashes identify bytes, not trustworthy reviewers.

The Linux/WSL x86_64 release and source are available. The walkthrough shows matching output, an ordering regression, and a repair. I'd appreciate feedback about setup friction and whether this fits a real command you maintain.

Repository: https://github.com/zhugez/invar
Walkthrough: https://github.com/zhugez/invar/blob/main/docs/first-run.md

### Personal invitation draft

I made a small local CLI called Invar to compare reviewed command output before and after code changes. Since you maintain CLI/scripts, would you be willing to try its disposable walkthrough and tell me where it gets confusing? No repository access or private output needed. If this job is already covered by your tests or snapshot tool, that feedback is useful too.

### Reply when someone gets stuck

Thanks for trying it. Which step stopped you, and what did you expect next? If you can share a reproduction, please use synthetic data and include your platform, `invar --version`, and the sanitized command/error. Do not attach raw baselines or private output. First-use form: https://github.com/zhugez/invar/issues/new?template=first-use.yml

## Owner launch gate

Review [readiness evidence](launch-readiness.md) before publication. This kit prepares recruitment; it does not send invitations, publish posts, deploy the site, set up payment, or promise support response times.
