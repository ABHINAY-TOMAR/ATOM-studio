# ATOM Studio Dependency Decisions

Status: **Provisional blueprint, not installed or validated.** Keep dependencies minimal until the desktop foundation spike and supported-target decision.

| Area | Candidate / decision | Why it fits current needs | Alternatives / risks |
|---|---|---|---|
| Desktop shell | Tauri 2 | Rust-owned backend, system webview frontend, native window/dialog/notification integration; separates UI from engine | Electron has broader bundled runtime but more memory/disk overhead; native GUI frameworks require a separate UI decision. Validate WebView2 and packaging/signing on target OSes |
| Frontend | React + TypeScript + Vite | Existing ecosystem and requested stack; fast local iteration and typed UI/core DTOs | Solid/Svelte/Vue; avoid adding state/query libraries until state flow proves need |
| Styling | Tailwind CSS + a small project-owned component/token layer | Consistent utility styling and custom ATOM visual identity | Avoid copying a component library's brand; choose accessible primitives only as needed |
| Async runtime | Tokio | Existing external network providers, streaming, bounded concurrent tasks and cancellation benefit from async I/O | Async-std; sync runtime. Tokio is recommended for ecosystem compatibility; do not put CPU-heavy rendering on async workers |
| HTTP client | Reqwest with Rustls | Provider and search APIs; TLS and connection pooling | Provider SDKs may supply client features; avoid duplicate HTTP stacks and configure redirect/timeout/SSRF policy |
| Serialization | Serde + serde_json | Typed IPC, event and persisted DTO representation | Validate untrusted input at boundaries; schema versioning remains app-owned |
| Errors | thiserror for core typed errors | Explicit error categories across task/provider/tool/persistence boundaries | anyhow only at executable boundary if useful; do not erase domain error types |
| Diagnostics | tracing + tracing-subscriber | Structured spans/events and correlation through async work | Redaction must be designed; never log prompts/keys by default |
| Persistence | rusqlite + bundled SQLite initially | Desktop local-first state, simple transactions and low operational burden | SQLx adds async compile/migration complexity; SQLite write operations can use a dedicated blocking worker. Choose SQLx only if measured concurrency/async needs justify it |
| Identifiers | UUID | Session/task/agent correlation IDs | ULID if sortable IDs materially help; sequence numbers still required for ordered events |
| CLI | clap only if a real diagnostics/import command is retained | Existing launcher/bootstrap features may need a native command surface | Do not build a parallel terminal product without a clear use case |
| Tests | cargo test; proptest selectively; Criterion only for measured hot paths | Unit/integration/security tests; reproducible benchmarks | Property tests and Criterion add maintenance; use for invariants/perf targets, not blanket adoption |
| Secrets | OS credential store adapter (target-specific, selected after platform decision) | Provider keys must not be stored in normal config/SQLite | Environment variables for development only; encrypted custom store is not a substitute for OS keychain without key management |

## Provider/integration dependency policy

OpenSwarm currently uses OpenAI/Azure-like/LiteLLM routing, Google GenAI, fal.ai, SearchApi and Composio-related integrations depending on capability. Do not add SDKs by assumption. For each adapter, first capture actual requests, capabilities, streaming, retries, errors, key requirements, side effects and current version constraints. Prefer Reqwest for stable, documented HTTP APIs where no maintained Rust SDK is required; retain mature external engines (FFmpeg/Office/browser) only where their existing behavior is necessary and package them deliberately.

## Toolchain and platform gates

Observed local tooling: Rust/Cargo 1.100.0, Node 26.13.1, npm 11.20.0, Python 3.13.17 on Windows. This is not a support matrix. Before pinning versions, verify Tauri 2 prerequisites, WebView2 behavior, Rust target support, installer/signing, target OS hardware and CI runners. Lock Node packages and Cargo dependencies for reproducible builds; run advisory/license review before release.
