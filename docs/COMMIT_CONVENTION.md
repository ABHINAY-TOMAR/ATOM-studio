# Branch and commit conventions

## Branches

Meaningful work starts from an up-to-date `main` on a short-lived branch:

```text
feat/<short-description>
fix/<short-description>
refactor/<short-description>
perf/<short-description>
security/<short-description>
docs/<short-description>
test/<short-description>
build/<short-description>
ci/<short-description>
```

Examples: `feat/agent-cancellation`, `fix/ipc-timeout`, `docs/release-process`. Keep each branch focused and linked to an issue. `main` is the stable integration branch; PRs are the normal merge path. Do not develop features directly on `main`, force-push, or rewrite published history. Delete merged branches when safe.

## Conventional Commits

Use:

```text
<type>(<optional-scope>): <imperative summary>
```

Types: `feat`, `fix`, `refactor`, `perf`, `security`, `docs`, `test`, `build`, `ci`, `chore`.

Examples:

```text
feat(runtime): implement agent lifecycle
fix(ipc): validate command arguments
refactor(core): isolate orchestration logic
perf(runtime): optimize task dispatch
security(tools): enforce execution permissions
docs: update installation guide
test(runtime): add cancellation tests
build: update workspace configuration
ci: add Windows build verification
chore: clean development scripts
```

Keep the subject concise, imperative, and free of ending punctuation. Scope is optional and names the affected component. Do not use vague subjects such as `update`, `fix stuff`, or `work in progress`; do not combine unrelated changes in one commit.

For an incompatible change, append `!` after the type/scope and include a `BREAKING CHANGE:` footer that explains the migration:

```text
feat(api)!: replace the task submission contract

BREAKING CHANGE: callers must pass a typed task request.
```

Commit tooling is intentionally not required; use this documented format and let review verify the change. Each PR should reference an issue and update the changelog for user-visible changes.
