# ATOM Studio Desktop Architecture

## Decision

Adopt **Tauri 2 + Rust + React + TypeScript + Vite + Tailwind CSS** as the preferred desktop stack for an initial validation spike. This is a design recommendation, not a completed platform validation. The repository currently has an npm/TUI product and Python API but no desktop UI. Local environment observed: Windows, Rust/Cargo 1.100.0, Node 26.13.1, npm 11.20.0, Python 3.13.17. Supported customer OS/hardware, WebView2 baseline, signing, GPU and installer constraints remain undecided.

## Process boundaries

```text
Tauri webview (React UI)
   │ typed commands / scoped events
   ▼
Tauri app adapter (window, dialogs, notifications, lifecycle)
   │ application-service calls
   ▼
Rust ATOM core (agents, tasks, tools, providers, policy, persistence)
   ├─ external provider APIs over validated TLS
   ├─ SQLite user state
   └─ explicitly selected external conversion engines / isolated workers
```

The UI must not call providers or integrations directly, manage secrets, decide authorization, or execute tools. Tauri commands are a narrow DTO-based API. Streams/events have task/session correlation IDs and redact sensitive fields. Windowing, tray behavior, deep links, notifications, auto-update and multi-window needs must be prioritized against MVP, not added by default.

## Desktop platform responsibilities

- Tauri owns windows, native dialogs, OS notifications, app lifecycle and packaging.
- React owns presentation and local transient view state only.
- Core owns durable sessions, policies, task state, tools and external I/O.
- User data lives under an OS-appropriate app-data root; workspace files remain at user-selected paths and are accessed through scoped grants.
- WebView2/runtime installation, code signing, update signing, macOS notarization and Linux packaging are release concerns requiring explicit target decisions.

## UI/core contract

Use versioned serializable command/query DTOs (`session.create`, `task.submit`, `task.cancel`, `activity.list`, `workspace.select`, `provider.configure` as examples, not API commitment). Use a typed event enum for task lifecycle, agent status, tool invocation/outcome, streamed response delta, artifact availability, notification and error. Include correlation IDs and monotonic sequence numbers to handle reconnect/resubscribe and ordering. Persist enough events/task state to recover the UI after a reload; do not make the event channel the only source of truth.

## Original app compatibility

The desktop app is the primary product interface. Preserve HTTP routes only if an external consumer needs them; the current `server.py` delegates route definitions to Agency Swarm and exact route contract is not established. Preserve CLI/bootstrap behaviors selectively when they serve installation, diagnostics or migration, not as a second UI. No Python bridge is assumed; retained Python engines must be explicit child processes with protocol, permissions, limits and failure handling.
