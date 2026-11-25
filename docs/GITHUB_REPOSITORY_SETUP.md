# GitHub repository setup checklist

This document records settings to apply in the GitHub web UI. The local Git remote can push, but GitHub CLI is not authenticated in the current environment, so repository settings, labels, milestones, and protections could not be inspected or changed. Inspect existing items before creating anything; create only missing entries.

## Labels

Open **Issues → Labels** and inspect existing labels first. Keep useful defaults, avoid duplicate names, and create only missing labels. Suggested colors are GitHub hex values without `#`:

| Type label | Color    | Type label      | Color    |
| ---------- | -------- | --------------- | -------- |
| `bug`      | `d73a4a` | `feature`       | `a2eeef` |
| `refactor` | `c5def5` | `performance`   | `fbca04` |
| `security` | `b60205` | `documentation` | `0075ca` |
| `testing`  | `bfd4f2` | `build`         | `5319e7` |
| `ci`       | `1d76db` | `maintenance`   | `ededed` |
| `question` | `d876e3` |                 |          |

| Priority label      | Color    | Status label         | Color    |
| ------------------- | -------- | -------------------- | -------- |
| `priority:critical` | `b60205` | `status:blocked`     | `b60205` |
| `priority:high`     | `d93f0b` | `status:ready`       | `0e8a16` |
| `priority:medium`   | `fbca04` | `status:in-progress` | `fbca04` |
| `priority:low`      | `0e8a16` | `status:review`      | `5319e7` |

| Area label     | Color    | Area label       | Color    |
| -------------- | -------- | ---------------- | -------- |
| `area:desktop` | `1d76db` | `area:rust-core` | `dea584` |
| `area:agents`  | `c5def5` | `area:providers` | `0e8a16` |
| `area:ui`      | `d876e3` | `area:security`  | `b60205` |
| `area:build`   | `5319e7` | `area:docs`      | `0075ca` |

| Other label           | Color    |
| --------------------- | -------- |
| `good-first-issue`    | `7057ff` |
| `needs-investigation` | `fbca04` |
| `breaking-change`     | `e99695` |

The existing question form uses the `question` label. Issue-form labels (`bug`, `feature`, `maintenance`) must exist before GitHub can apply them. Use **Issues → Milestones** to inspect current milestones before adding any; create only missing entries from the table below.

## Milestones

Inspect the repository's existing milestones and create missing milestones with these names and statuses:

| Milestone                                  | Status    |
| ------------------------------------------ | --------- |
| M0 — Reverse Engineering & Planning        | Completed |
| M1 — Desktop Foundation                    | Completed |
| M2 — Production Development Infrastructure | Current   |
| M3 — Rust Core Foundation                  | Planned   |
| M4 — AI Provider Infrastructure            | Planned   |
| M5 — Agent Runtime & Orchestration         | Planned   |
| M6+ — Existing OpenSwarm Feature Migration | Planned   |

Close M0 and M1 only if their completion is confirmed against the existing project history; keep M2 open as current and M3 onward open/planned. Do not invent due dates. Link issues to the milestone that owns their acceptance criteria.

## Main branch rules

In **Settings → Rules → Rulesets** (or branch protection rules where rulesets are unavailable), target `main` and configure:

- Require a pull request before merging.
- Require the `Rust workspace (headless)` and `Frontend (build and dependency audit)` checks from `ATOM Studio Desktop CI`.
- Require conversation resolution.
- Block force pushes and branch deletion.
- Do not require approving reviews while the repository is solo-maintained; that would make the PR gate impossible without a second trusted maintainer. The maintainer still performs a full self-review and independent OpenCode Review.
- Do not require signed commits unless signing is configured and verified.

Confirm rules are active in GitHub settings after saving. If the account/repository plan does not support private-repository rulesets, document that limitation and continue to follow the PR-only convention; do not claim `main` is protected.

## Security settings

In **Settings → Code security and analysis**, enable Dependabot alerts and security updates when available. Enable private vulnerability reporting so the link in `.github/ISSUE_TEMPLATE/config.yml` works. The security entry is a private contact link rather than a public issue form, to avoid prompting disclosure of exploit details in public. Enable secret scanning/push protection if offered for this repository plan. Confirm the settings in the UI; the local workflow does not prove those account-level controls are active.

The CI currently audits npm dependencies at the high severity threshold. Dependabot supplies dependency update/advisory PRs when repository alerts are enabled. There is no dedicated RustSec `cargo audit` job, automated secret scanner, or license-policy audit in this milestone; do not treat those checks as present. A license allowlist has not been agreed, so avoid a noisy speculative policy gate. The repository carries an MIT license. Do not print credential values in CI logs.

## GitHub access blocker and verification

Current checks observed: `gh` is installed, but `gh auth status` reports no authenticated GitHub host; GitHub REST reads for repository metadata, labels, milestones, and branch protection were denied/not found. Git transport authentication only proves push access, not GitHub API permissions.

After an authorized maintainer completes `gh auth login`, inspect the actual repository and settings before administering them. Then record which labels/milestones were created, and verify branch/security settings directly in GitHub. Do not infer successful administration from a successful local push.
