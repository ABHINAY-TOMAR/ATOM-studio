# ATOM Studio MVP Acceptance Criteria

MVP scope must be finalized against the feature matrix and supported platform decision. These criteria define a practical first product slice while preserving migration direction.

## Product and desktop

- Installs and launches as a desktop application on the explicitly supported target OS/hardware matrix.
- ATOM Studio identity, original dark-first design system, keyboard-accessible navigation, workspace selection and first-run settings are present.
- UI and core communicate through typed, validated Tauri commands/events; UI contains no provider secrets or tool authorization logic.
- Responsive main work surface supports a session, streaming response, task status, activity inspection, cancellation, empty/error/loading states and reopening recent sessions.

## Core behavior

- At least one representative existing OpenSwarm agent workflow is end-to-end functional with a real configured provider; no fake success path.
- Existing orchestrator/specialist handoff behavior selected for MVP is described in tests and events.
- Provider setup validates availability, redacts credentials, reports errors, and supports cancellation/timeouts.
- Tools are explicitly granted, schema-validated, bounded and audited. Risky external writes require enforced confirmation.
- User-selected files/workspaces are constrained to granted roots; traversal and symlink behavior is tested.
- Task concurrency, queueing, context, response and tool-output limits are configurable or safely bounded.

## Persistence and recovery

- Session/task history survives app restart with schema migration and recoverable database behavior.
- App shutdown does not silently report unfinished tasks complete; cancellation/recovery state is visible.
- Generated artifact references and paths are discoverable in the session/workspace UI.

## Quality/security

- Rust formatting, workspace check/test/clippy and frontend typecheck/test/build pass in CI.
- Security suite covers permissions, secrets, path traversal, SSRF policy, IPC validation, resource limits and cancellation for enabled features.
- Shell/Python execution stays disabled unless actual isolation is implemented and separately approved.
- No future features (Voicebox, AutoClip, additional studios) are present in navigation, dependencies or runtime.
- Performance report contains measured baseline and Rust measurements with environment/methodology; no improvement claim without evidence.

## Compatibility disclosure

Every migrated feature has a matrix status, test evidence, known differences and required external engine/provider. The MVP explicitly identifies features not yet migrated rather than presenting nonfunctional controls.
