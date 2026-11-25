# ATOM Studio Migration Roadmap

Each phase is gated by a reviewed artifact and tests. Existing OpenSwarm remains untouched as the behavior reference. Order may change after API/engine capture.

| Phase | Deliverable | Exit gate |
|---|---|---|
| 0. Decisions and repository preparation | Scope/ADRs, version-controlled migration location, target OS/hardware and feature/API decisions | No destructive source changes; reproducible branch/worktree |
| 1. Characterization | CLI/API/provider/tool contracts, sanitized cases, artifact corpus, initial perf/security baselines | Existing behavior runs and captures are repeatable |
| 2. Workspace and desktop foundation | Cargo workspace, Tauri shell, React/TS/Vite/Tailwind, build pipeline | Build/package smoke on agreed first target; core independent from UI |
| 3. Design system and desktop shell | ATOM tokens, navigation, workspace/session shell, accessibility states | Reviewed UX, UI tests, no placeholder features exposed |
| 4. Core domain/runtime | IDs/errors/config, bounded task lifecycle, cancellation/timeouts/events | Unit/concurrency tests and quality gates pass |
| 5. Providers | Capability/error/stream interfaces plus first current provider adapters | Contract tests and secret-redaction checks |
| 6. Agents/orchestration | Agent roster, orchestrator routing, handoffs, specialist lifecycle | Replay of representative existing interactions |
| 7. Tools/security substrate | Registry, schemas, grants, approvals, path/network policies, audit events | Denial and abuse tests pass; no unsafe execution enabled |
| 8. Research/virtual assistant | Search and existing connected-service/file operations | Per-tool permission and side-effect parity tests |
| 9. Data analysis | Decide bounded native workflows vs isolated Python worker; migrate supported behavior | Resource/isolation tests; no arbitrary in-process execution |
| 10. Documents/slides | Existing document and PPTX capability with retained engines where required | Golden outputs, platform dependency and recovery tests |
| 11. Existing image/video/audio | Migrate current operations/providers only | Provider and artifact tests, size/time/resource limits |
| 12. Persistence/config/history | SQLite migrations, sessions, settings, state-root migration | Backup/recovery, schema migration and secret storage tests |
| 13. Compatibility and packaging | Necessary CLI/API compatibility, import/migration flow, installers | Supported target matrix, signed artifact and end-to-end suite |
| 14. Hardening and performance | Threat tests, profiled optimization, baseline comparison | No unverified security claims; publish reproducible performance report |
| 15. Release readiness | Docs, support matrix, recovery/rollback, release pipeline | MVP acceptance criteria all pass |

## Initial critical path

Desktop build spike → desktop shell/design contract → task/event model → provider streaming → first agent workflow. Do not build media integrations before the provider/tool security boundary and task lifecycle exist. Heavy conversions and Python analytics are separate integration decisions, not assumed Rust ports.

## Explicitly deferred

Voicebox, AutoClip, new creative studios/engines, additional agents, distributed deployment and generic plugin marketplace remain future roadmap items and must not enter current implementation phases.
