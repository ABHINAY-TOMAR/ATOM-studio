# Feature Migration Matrix

Statuses describe the ATOM Studio migration, not the completeness of OpenSwarm. Current status is **Not analyzed** unless marked otherwise. “Analyzed” means static behavior inventory only; none of these features is implemented in Rust yet.

| Feature | Original source | Proposed ATOM implementation | Status | Required proof / compatibility notes |
|---|---|---|---|---|
| Roster and agent config | `swarm.py`, agent factories/instructions | Typed agent definitions/config in core; persisted settings separate from executable policy | Analyzed | Verify defaults, model, instructions, per-agent tools |
| Orchestrator | `orchestrator/`, `swarm.py` | Runtime strategy using task graph, bounded execution, explicit delegation policy | Analyzed | Characterize message vs handoff, failure/cancel semantics |
| Specialist lifecycle | `*/ *_agent.py` | State machine and bounded invocation managed by runtime | Analyzed | Capture framework lifecycle and streaming behavior |
| Provider/model routing | `config.py`, agent factories | Provider adapters, capability discovery, secure key references | Analyzed | Verify OpenAI/LiteLLM and each active media/search provider separately |
| Tool framework | `*/tools`, `shared_tools/` | Typed schemas, permission/capability grants, validators, timeout/audit | Analyzed | Inventory every tool schema, side effect, and registration |
| Virtual assistant | `virtual_assistant/` | Native integration adapters where available; explicit confirmation gates for writes | Analyzed | Composio API equivalence is unresolved; all writes require policy tests |
| Research | `deep_research/`, `ScholarSearch.py` | Provider-backed search tools and source-aware results | Analyzed | Query privacy, citations, timeout, rate-limit behavior |
| Data analysis | `data_analyst_agent/` | Rust-native common formats/operations only if fidelity is practical; otherwise isolated Python worker with explicit consent and resource limits | Analyzed | No in-process generated code; specify supported formats/operations and sandbox before enabling |
| Documents | `docs_agent/` | Rust orchestration around retained/selected conversion engines; secure file roots | Analyzed | Golden artifact corpus; determine which native converters are retained |
| Slides | `slides_agent/` | Rust job orchestration plus validated existing PPTX/rendering engines where needed | Analyzed | Preserve PPTX behavior; browser/native runtime packaging risk |
| Images | `image_generation_agent/` | Provider adapters and controlled file/job output | Analyzed | Verify each endpoint, reference URL limits and key support |
| Video/audio | `video_generation_agent/` | Async job/tool orchestration, external providers and bounded local media processing | Analyzed | Preserve existing operations only; no AutoClip/future studio |
| Files | various tools, `mnt`, uploads | Workspace-scoped filesystem service, explicit grants and atomic writes | Analyzed | Path traversal, symlink and overwrite behavior tests |
| Context/history | Agency Swarm and persistent tools | Versioned SQLite persistence, bounded context and session history | Not analyzed | Framework's current schema and retention unknown |
| CLI/bootstrap | `bin/`, `run_utils.py` | Native installer/desktop bootstrap only as needed; don't retain Python dependency bootstrap by default | Analyzed | npm launcher behavior and supported OS matrix need capture |
| HTTP/API | `server.py`, Agency Swarm integration | Desktop IPC as primary boundary; optional compatibility HTTP API only if required | Analyzed | Exact route/API contract not captured; do not expose localhost API without auth |
| Branding/product env | `openswarm.config.mjs`, product JSON | ATOM Studio identity/config migration, preserve import of existing user settings where appropriate | Analyzed | State-root and existing config migration test |
| Tracing/logging | `swarm.py`, Agency Swarm SDK | `tracing`, redaction and user-visible activity events | Not analyzed | Existing event/log contract and trace behavior unknown |
| Installer/release | workflows, Docker, npm platform packages | Tauri packaging and signed releases for agreed platforms | Analyzed | Native prerequisites and update signing/hosting to decide |

## Migration policy

Preserve capabilities, not accidental implementation details. Intentional safety changes (for example, refusing arbitrary paths or requiring confirmation for consequential connected-service actions) must be listed as behavior differences and covered by tests. Never label a feature “implemented” because it has a UI control or trait; status becomes **Implemented** only with functioning code, **Tested** with executed tests, and **Verified** after behavior/artifact/API compatibility checks pass.
