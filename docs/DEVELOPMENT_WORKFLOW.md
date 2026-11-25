# Development workflow

This guide describes the repository workflow for the ATOM Studio migration. It does not change the still-preserved OpenSwarm reference source or imply that its behavior is migrated.

## Local setup and verification

Clone the repository and install the desktop frontend dependencies:

```sh
git clone https://github.com/ABHINAY-TOMAR/ATOM-studio.git
cd ATOM-studio
cd rust/apps/desktop
npm ci
npm run tauri dev
```

Frontend build and IPC smoke test (the IPC test requires a Rust desktop build first):

```sh
npm run build
cd ../..
cargo build --workspace
cd apps/desktop
npm run verify:ipc
```

Rust quality checks, from `rust/`:

```sh
cargo fmt --all -- --check
cargo check --locked --workspace
cargo test --locked --workspace
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo build --locked --workspace
```

`npm run verify:ipc` launches Vite and a real Tauri/WebView2 window on Windows. It is a local GUI smoke test; the new GitHub workflow is configured for separate headless Rust and frontend jobs and does not claim to launch the GUI. Those GitHub checks remain unverified until the workflow runs.

## Issue-to-merge lifecycle

1. Open an issue using the GitHub issue form with the scope and acceptance criteria; use `gh issue create --title "feat: short summary" --body "Problem, proposal, and acceptance criteria"` when the CLI is authenticated. Link existing issues rather than duplicating them. Report vulnerabilities through the private channel in `SECURITY.md`, never in a public issue.
2. Start from an up-to-date `main` and create a short branch named `<type>/<description>`; see [branch conventions](COMMIT_CONVENTION.md). Example: `git fetch origin`, `git switch main`, `git pull --ff-only`, `git switch -c feat/short-description`.
3. Implement only the issue's approved scope. Keep original OpenSwarm behavior and code intact unless the issue explicitly authorizes a migration change.
4. Run relevant local checks and record exact commands/results. For UI changes, capture screenshots and test responsive states; run the local IPC smoke test when the desktop shell or IPC is affected.
5. Request independent OpenCode Review on the complete diff against the linked issue. Review each finding with code evidence; fix confirmed issues, or document why a finding is unsupported. The local environment has no verified `opencode` CLI/API, so use the installed review product's verified UI/integration manually. Do not make CI depend on an assumed command.
6. Re-run affected checks after fixes; rerun the full local verification set for changes that cross Rust, frontend, or build boundaries.
7. Stage only intended paths with `git add <path>`, inspect `git diff --cached`, and commit with a Conventional Commit subject (for example `git commit -m "feat(ui): add workspace settings"`). Push with `git push -u origin feat/short-description`. Open the PR with `gh pr create --base main --head feat/short-description --web` or GitHub's **Compare & pull request** flow; complete the PR template and use `Closes #ISSUE_NUMBER`. Wait for CI and review. A clean build alone is not approval.
8. Merge only after acceptance criteria pass, CI is green, critical/high findings are resolved or explicitly accepted with evidence, and conversations are resolved. For a solo maintainer, do not require a fictitious second reviewer; self-review the full diff before merging. If the configured merge method allows squash, an authenticated maintainer can use `gh pr merge <number> --squash --delete-branch`; otherwise use the repository's allowed merge method in GitHub.
9. Delete the merged branch when GitHub offers it and it is no longer needed. Never force-push or rewrite published history.

## OpenCode Review: independent review protocol

OpenCode Review is an independent engineering aid, not an automatic approval gate and not a substitute for tests, CI, or human judgment. Review the full branch diff and linked issue, and request a structured report for correctness, security, Rust practices, architecture, performance, concurrency, error handling, memory safety, UI behavior/accessibility, tests, regressions, requirements, scope, and dependency risks.

Each finding must include severity (`Critical`, `High`, `Medium`, `Low`, or `Informational`), file and location, finding, evidence, impact, recommended correction, and whether it is confirmed, potential, speculative, or a suggestion. Reject vague claims by requesting evidence. The reviewer should not edit files during review unless a separate fix workflow is explicitly authorized.

The implementer records each finding and disposition, fixes confirmed issues, documents evidence-based rejections, and re-verifies changes. Request another review after substantial corrections. Do not merge with unresolved critical issues, unexplained high issues, failing checks, or unmet acceptance criteria.

No OpenCode Review executable was discoverable in the current development environment. Its version, CLI, GitHub integration, and automation capabilities have not been verified. Keep review manual until maintainers verify the installed product's documented interface; never invent a command or API.

## Scope

The workflow targets `windows-2022` to exercise the Windows build in GitHub-hosted CI. The local development build was verified on Windows 10 using the GNU toolchain; hosted MSVC results are pending the first workflow run. The workflow performs headless compilation and frontend checks; local WebView2 launch/IPC remains a separate check. No macOS or Linux support claim is made.
