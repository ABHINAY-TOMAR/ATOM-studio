# ATOM Studio Project Scope

## Product

**ATOM Studio** is a Rust-powered desktop evolution of OpenSwarm. OpenSwarm remains intact as the source-of-truth reference while its existing capabilities are migrated into a redesigned desktop application.

## In scope

- Re-engineer currently shipped OpenSwarm behaviors: eight-agent roster, orchestration, supported provider/model routing, tool operations, research, connected services, data analysis, document/slide/image/video/audio workflows, local files, configuration, logs, history/context, and API/CLI behavior where it remains relevant to desktop operation or compatibility.
- Deliver an original desktop UX with workspace/project context, conversations, agent/task/tool activity, artifact access, provider/integration setup, settings, notifications, and session history.
- Improve security and resource control with enforceable authorization, path/network policies, bounded execution, cancellation, timeouts, and redacted observability.
- Keep a modular monolith with explicit boundaries that can evolve without speculative infrastructure.

## Out of scope

Voicebox, AutoClip, future creative studios, new creative engines, unrelated products, new agent systems, speculative plugin marketplaces, distributed microservices, and unimplemented extensibility scaffolding. Existing image/video/audio capability is in scope; new engines and future studio products are not.

## Preservation and delivery

Keep the Python/Node OpenSwarm source, dependencies, config, packaging and docs unchanged as reference. Place new Rust/Tauri implementation under `rust/` in the existing workspace, subject to confirming that the checkout is a disposable/non-shipping source tree; otherwise use a sibling checkout. The current workspace has no `.git` metadata, so establish a version-controlled migration branch/worktree before production implementation.

## Working assumptions to verify

- Primary initial desktop target: Windows, inferred from the current development environment only. It is not an approved cross-platform support commitment.
- UI: Tauri 2 + React + TypeScript + Vite + Tailwind, subject to a minimal platform build and packaging spike.
- First releases retain current OpenSwarm integrations and feature behavior; changes needed for enforceable security are called out and tested as intentional compatibility changes.
- API keys and user state are stored in OS-appropriate secure storage/configuration; no credentials are committed or sent to UI logs.

## Exclusions from current architecture

No Voicebox/AutoClip models, routes, dependencies, pages, agent definitions, engines, or placeholder extension points. Extensibility means clean current interfaces (provider/tool/event seams) only where multiple existing implementations or an actual current requirement needs them.
