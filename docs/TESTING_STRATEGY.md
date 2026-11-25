# ATOM Studio Testing Strategy

## Evidence baseline

The source audit found two Python test files (`tests/test_html_docx_images.py`, `tests/test_slides_internal_models.py`), plus bootstrap/platform/live smoke scripts and CI workflows. This is not enough to characterize all features. Current API route and provider behavior are partly delegated to dependencies.

## Test layers

- **Characterization/replay:** capture original CLI behavior, API routes/streaming, tool inputs/outputs, side effects, and representative generated artifacts before replacing a subsystem. Use sanitized fixtures; never record real secrets or private customer data.
- **Rust unit:** lifecycle transition invariants, config parsing, policy decisions, path/network validation, context caps, error classification and serialization.
- **Integration:** task → orchestrator → specialist → provider/tool → event/persistence → UI contract, with deterministic fake provider only for test isolation (never production fake success).
- **Contract:** provider schemas/error/rate-limit/streaming, Tauri command/event DTO versions, Composio actions and any retained conversion engine.
- **UI:** accessible component tests, navigation/empty/loading/error states, keyboard flows, activity event ordering and cancellation.
- **End-to-end:** packaged desktop smoke, first-run configuration, workspace grant, real provider opt-in, generation/artifact open, restart and history recovery.
- **Security:** invalid permissions, prompt injection through tool output, traversal/symlink edge cases, SSRF/redirect/private IP, secret redaction, IPC misuse, malformed payloads, resource limits and cancellation.
- **Concurrency/stress:** bounded queues, provider/tool failures, shutdown, high task count, large context/artifacts and slow consumers.
- **Performance:** baseline-plan workloads in release profile, report environment and reproducible results.

## Release gates

Run `cargo fmt --all --check`, `cargo check --workspace`, `cargo test --workspace`, and `cargo clippy --workspace --all-targets --all-features -- -D warnings`, plus frontend typecheck/lint/test/build and Tauri packaging smoke on every supported OS. Run relevant integration/security tests before enabling each capability. Do not represent mock-based tests as provider compatibility verification.

## Safety and determinism

Use isolated test workspaces and temporary credentials; provider integration tests are explicit opt-in. Mock time/IDs only at seams. Tests must assert redacted logs and that cancellations stop child work. Golden documents/slides/media should compare stable structure and required properties rather than nondeterministic bytes where engines embed timestamps.
