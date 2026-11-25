# Rust Workspace Structure

## Proposed initial layout

```text
rust/
├── Cargo.toml                 # workspace, shared package metadata
├── Cargo.lock
├── rust-toolchain.toml        # pin only after platform/toolchain spike
├── apps/
│   └── desktop/
│       ├── ui/                # React + TS + Vite + Tailwind
│       └── src-tauri/         # Tauri shell and thin IPC adapter
├── crates/
│   └── atom-core/             # runtime/domain/application services initially
├── tests/                     # integration and security tests, when needed
├── docs/                      # migration map, ADRs, behavior captures
└── scripts/                   # reproducible local build/package tasks
```

## Initial crate boundaries

- `atom-core`: one library containing domain types, use cases, runtime, provider/tool boundaries, policy, events and SQLite persistence initially. Private modules, narrow public API.
- `atom-desktop` (Tauri package): window lifecycle, IPC DTOs, notifications/dialogs and UI event adaptation. It depends on `atom-core`; core never depends on Tauri or frontend code.
- `ui`: presentation-only client. No provider credentials, domain policy, or direct integrations.

Do not create separate crates for every conceptual subsystem at the start. Extract `atom-providers`, `atom-tools`, or similar only when dependency/build isolation or a stable testable boundary provides an actual advantage.

## Dependency direction

`ui → Tauri commands → desktop adapter → atom-core → ports/adapters → external services`.

`atom-core` must not depend on Tauri, React, WebView, or concrete media UI. External-engine adapters are invoked through explicit bounded process/service boundaries where required. Original OpenSwarm stays outside the workspace under its current source tree.

## Source placement and preservation

This checkout currently has no `.git` metadata. Before implementation, establish a real repository/branch or create a separate migration checkout. The proposed `rust/` directory is suitable only if the existing root is safely version-controlled and packaged artifacts exclude it until intentional. Do not move or rewrite original modules as part of scaffolding.
