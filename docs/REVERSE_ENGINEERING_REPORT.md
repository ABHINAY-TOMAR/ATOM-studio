# OpenSwarm Reverse-Engineering Report

**Scope:** Source and configuration audit of the checked-out OpenSwarm tree. The original repository remains the reference implementation; no Rust code has been added in this phase.

## 1. System overview

OpenSwarm is an AI-agent application and distributable launcher, rather than a standalone agent implementation. The application assembles eight Agency Swarm agents (one orchestrator and seven specialists), equips them with framework-provided and project-specific tools, and exposes a terminal TUI and a FastAPI integration. Specialized workflows cover productivity and connected services, research, data analysis, documents, slides, images, and video.

The repository combines Python application code, a Node/npm launcher and TUI packaging workflow, external AI/service APIs, local file-producing tools, and media/document conversion dependencies. A Rust rewrite therefore needs to reproduce an application boundary and replace framework and tooling behavior, not just translate a small deterministic core.

## 2. Feature inventory

| Area | Observed functionality | Key paths |
|---|---|---|
| Orchestration | Creates eight agents; orchestrator can message each specialist; all distinct agent pairs can hand off | `swarm.py`, `orchestrator/` |
| Virtual assistant | Email, calendar, Slack and local-file actions; dynamic Composio tool discovery/connection/execution | `virtual_assistant/`, `shared_tools/`, `helpers.py` |
| Research | Web and scholarly search, synthesis with citations; Python interpreter is registered | `deep_research/`, `virtual_assistant/tools/ScholarSearch.py` |
| Data analysis | Spreadsheet/data analysis, statistical and charting workflows, persistent Python/Jupyter execution, external connected-data actions | `data_analyst_agent/`, `requirements.txt` |
| Documents | HTML/Markdown creation, preview, listing, viewing, editing, conversion to DOCX/PDF/Markdown/TXT, snapshot/restore support | `docs_agent/` |
| Slides | HTML-based slide creation/editing, checks/previews, image sourcing, PPTX export and related utilities | `slides_agent/` |
| Images | Provider-backed image generation/editing, composition and background removal | `image_generation_agent/` |
| Video/audio | Generation, editing, trimming/combining, image-to-video utilities, audio replacement and subtitle workflows | `video_generation_agent/` |
| Shared integrations | Composio session/tool helpers, discovery, connection management, file copying, model-availability helpers | `shared_tools/`, `helpers.py` |
| Launcher/bootstrap | npm platform TUI binary resolution, Python environment/bootstrap, optional native/media/browser dependency installation, product configuration | `bin/`, `run_utils.py`, `scripts/`, `openswarm.config.mjs` |
| Deployment | Python FastAPI server and Docker image/compose configuration | `server.py`, `Dockerfile`, `docker-compose.yml` |

Feature descriptions in `instructions.md` are agent policies and intended workflows. They are not equivalent to enforced runtime guarantees; availability, limits, approvals, and retries must be checked in tool code and provider APIs.

## 3. Architecture map

```text
User
 ├─ npm `openswarm` launcher → platform AgentSwarm TUI binary
 │                            → Python OpenSwarm runtime
 ├─ `python swarm.py` ───────→ Agency Swarm TUI
 └─ `python server.py` ──────→ Agency Swarm FastAPI integration (:8080)
                                      │
                              `swarm.create_agency()`
                                      │
                ┌─────────────────────┴─────────────────────┐
                │ orchestrator + 7 specialist agents        │
                │ instructions, context, registered tools    │
                └─────────────────────┬─────────────────────┘
                                      │
           Handoff edges between every pair of agents; orchestrator
           has SendMessage edges to each specialist
                                      │
          Agency Swarm / OpenAI Agents SDK / model provider
                                      │
         Composio, search APIs, media APIs, local filesystem,
         Python/IPython/shell execution, document/media converters
```

`swarm.py` is the composition root. Agent factories are spread across each agent package. Tool implementations are generally in per-agent `tools/` directories, with shared Composio helpers in `shared_tools/` and cross-cutting behavior in `helpers.py` and `config.py`. There is no project-owned database/persistence layer visible in the audited application; conversation/thread persistence is delegated to Agency Swarm callbacks/runtime.

## 4. Execution flow

1. The npm command resolves a platform-specific AgentSwarm executable and passes through arguments, or the Python command bootstraps dependencies and configures the terminal runtime.
2. Runtime setup loads environment/product configuration, applies UTF-8 file-read and IPython/Composio patches, and configures/disables tracing according to the available OpenAI key.
3. `create_agency()` creates the orchestrator and seven specialists using their factories, then registers shared instructions and communication flows with `Agency`.
4. The orchestrator prompt routes work to specialists. `SendMessage` is available from orchestrator to specialists; `Handoff` is wired for all distinct pairs. The policy for choosing between handoff and message is prompt-defined.
5. A specialist invokes registered tools. Depending on the agent this can call external APIs, connected Composio actions, local files, subprocess/browser/media tooling, or persistent Python execution.
6. Terminal execution uses Agency Swarm TUI. `server.py` invokes the framework's FastAPI integration with the agency factory on port 8080, logging enabled, and `./uploads` as an allowed local-file directory.

Detailed internal scheduling, retry semantics, streaming protocol, thread storage, and HTTP route schemas are owned by pinned Agency Swarm/Agents SDK dependencies; they are not implemented as first-party logic in this repository and require separate contract capture before replacement.

## 5. Dependency analysis

### Python/runtime dependencies

`pyproject.toml` requires Python >=3.12 and declares Agency Swarm (pinned to 1.10.5), FastAPI/Uvicorn, Composio packages, dotenv, data science/Jupyter libraries, document and office processing, browser rendering, and image/video generation dependencies. `requirements.txt` separately declares a largely overlapping set; installation paths can therefore resolve different versions. `requirements-dev.txt` includes pytest.

Notable runtime stacks include:
- Agency Swarm/OpenAI Agents SDK for agents, tools, TUI, tracing and FastAPI integration.
- Composio and `composio-openai-agents` for external service discovery/actions.
- pandas/NumPy/SciPy/statsmodels/scikit-learn, plotting, and Jupyter/IPython for analysis.
- Playwright, WeasyPrint, python-docx, Poppler/LibreOffice and HTML helpers for document/slides workflows.
- Google GenAI, OpenAI, fal.ai, Pillow, MoviePy/FFmpeg and related native/system libraries for media.

### Node/npm dependencies

`package.json` exposes `bin/openswarm`, pins `@vrsen/agentswarm` to 1.4.43 and includes Playwright, presentation-generation libraries, React, Sharp, and patch-package. Optional platform packages supply AgentSwarm TUI binaries. Release workflows build/publish platform-specific npm packages and the main package.

### Build and delivery

The Docker image is based on Python 3.13 slim and installs Node, Python packages, npm packages, browser/system libraries, LibreOffice and Poppler; compose mounts `mnt` and `uploads`, injects `.env`, and exposes 8080. GitHub workflows cover launcher/bootstrap checks, selected live smoke tests, macOS tests, and npm releases. This is not a broad server/API cross-platform integration matrix.

## 6. Component relationships

- `swarm.py` imports the factories and owns the full connectivity graph.
- Agent factories define tool registration and load per-agent instructions. Tool schemas and lifecycle are coupled to Agency Swarm/Pydantic.
- `helpers.py` obtains Composio clients/sessions and runs discovered actions; `shared_tools/` wraps these operations.
- Agent instructions and shared instructions provide routing, workflow and safety guidance but do not enforce capability boundaries.
- `config.py` selects/resolves models using environment settings and Agency Swarm model types.
- `run_utils.py` owns much of install/bootstrap, state-root/product-env handling, platform detection, CLI presentation and optional dependency installation.
- `server.py` delegates HTTP behavior to Agency Swarm's FastAPI integration; no first-party route definitions were identified in the entry point.

A repository analysis found 142 Python source files, approximately 21,355 Python LOC, two Python test files, 30 analyzer units (12 ready, 17 caution, 1 blocked), and no parser errors. It also identified broad import cycles across the runtime and agent packages, so per-agent line-by-line ports are not independent.

## 7. Data flow and side effects

User prompts and conversation context flow through the Agency Swarm runtime and model APIs to the orchestrator and specialists. Specialist tools may then:
- transmit prompts, search queries, documents, images, or other inputs to OpenAI, Google, fal.ai, SearchApi, Composio, or connected third-party services;
- read/write local project files, `mnt`, `uploads`, or caller-selected copy destinations;
- execute Python/IPython or shell work in persistent tool environments;
- create documents, slide decks, images, videos and intermediate artifacts;
- mutate email, Slack, calendar, labels, or other connected-service state.

Secrets are supplied through environment/state configuration rather than committed source. Tool/network data classification, retention, and audit behavior depend partly on third-party services and framework internals.

## 8. API and entry-point inventory

| Interface | Behavior |
|---|---|
| npm CLI | `openswarm` maps to `bin/openswarm`; resolves OS/architecture-specific AgentSwarm binary and forwards arguments. Version flags are handled by launcher. |
| Python CLI/TUI | `python run_utils.py`/installed `openswarm` bootstraps optional dependencies and launches a demo; `python swarm.py` creates the agency and opens Agency Swarm TUI. |
| HTTP | `python server.py` invokes Agency Swarm `run_fastapi` with agency name `open-swarm`, port 8080, logging enabled, and local-file access configured for `./uploads`. Actual route contract is delegated to that dependency and was not inferred from this wrapper. |
| Slide preview server | `slides_preview_server.py` is a separate local preview entry point; its routes were not fully characterized in this pass. |
| Package entry | `pyproject.toml` exposes `openswarm = "run_utils:main"`. |

No claim is made here about exact generated HTTP routes or wire compatibility; record those from the pinned framework before replacing it.

## 9. Existing limitations and observed maintenance risks

- Agent routing, approval, retry, and tool policy are often expressed in prompts rather than mechanically enforced constraints.
- High-capability tools include persistent Python/shell execution and arbitrary named Composio actions; no general process sandbox was established by this audit.
- The analyzer reports broad import cycles and one blocked unit; 17 additional units need caution, notably large document conversion utilities and plugin-shaped exports.
- The analyzer marks many interfaces as Python/plugin-object shaped, meaning a pure Rust port requires designing new data/API contracts instead of preserving a callable in-process API.
- Python dependency lists overlap and are not uniformly locked. npm packaging also has platform and version coupling.
- The checked-in tests are sparse relative to the feature surface. Tests include document image handling and slide model/stream behavior; bootstrap checks live in scripts/CI.
- Agent prompts may describe requirements that implementation/framework behavior does not guarantee.
- `data_analyst_agent/tools/__init__.py` references exports not present in that folder according to the source audit; verify whether this stale package export is used.

These are audit findings, not a complete defect inventory; no production execution or security penetration test was run.

## 10. Performance bottlenecks and costs to investigate

No benchmark or profiler run was performed, so no measured bottleneck or speedup is claimed. Likely latency/cost sources to measure include external model round trips and multi-agent handoffs, Composio/API latency, persistent interpreter startup/contention, browser/Office rendering, large media transfers and transcoding, Python bootstrap/system package installation, and serially dependent workflow steps. Rust would not eliminate remote service latency or replace native media/rendering engines automatically. Establish workload-specific baselines before choosing performance targets.

## 11. Security weaknesses and threat observations

This is a source-level risk assessment, not a formal penetration test.

- **Execution:** IPython, persistent shell and interpreter tools provide broad execution capabilities. No isolated sandbox guarantee was found in the inspected application code.
- **Tool authorization:** Some instructions ask for user confirmation, but a prompt alone is not an authorization boundary. Composio actions can have account-level side effects.
- **Filesystem:** The API allows `./uploads`; tools read/write files and shared `CopyFile` accepts a caller-selected destination. Document project/name-to-path handling merits containment validation. Test coverage for one local-image containment helper exists, but does not establish global path safety.
- **Network/SSRF:** Research and image/media tools call remote services and may process caller-supplied URLs. URL allowlists, redirect handling, private-network restrictions, and response-size constraints must be verified per tool.
- **Secrets:** Provider/API credentials are environment-based, but tool output, errors, subprocesses and traces need explicit redaction review. Runtime tracing has special API-key handling in `swarm.py`.
- **Resource exhaustion:** Media conversion, large documents, persistent interpreters and concurrent agents may consume significant CPU, memory, storage or provider quota; limits need explicit capture/enforcement.
- **Deployment:** Docker exposes the API port and mounts host directories. The inspected Dockerfile does not define a non-root user or health check. Authentication and network exposure behavior are delegated to the framework integration and must be verified before public deployment.
- **Supply chain:** Python/npm dependencies, platform binaries, native tools and release workflows create a broad supply chain. Locking, provenance, vulnerability scanning and pinned toolchain versions should be part of migration.

## 12. Migration risks

1. **Framework replacement:** Agency Swarm owns agent lifecycle, conversation/context, tool schemas, handoffs, streaming, terminal UI and FastAPI integration. Recreating these behaviors is the largest compatibility risk.
2. **External integrations:** Composio provides dynamic toolkit/action discovery and account connections. Rust support and equivalent schemas/permissions must be verified; a hand-written HTTP wrapper is not automatically equivalent.
3. **Python execution:** Data analysis and some agents rely on persistent Python and shell tools. Replacing them with Rust libraries changes user capabilities; retaining them requires a separately designed, isolated compatibility boundary (not an in-process fake).
4. **Media/document functionality:** Office/HTML/PDF/browser/image/video tasks rely on native libraries, browser runtimes and external providers. Keep mature converters where sensible or port behavior around them explicitly; native Rust replacement of every engine is not implied.
5. **Behavioral contract:** Prompts, generated artifacts, file paths, versions, tool outputs, streaming and error behavior need characterization cases. Two tests cannot establish parity for the full product.
6. **Packaging:** The npm launcher and 12 platform binary packages are product functionality. A Rust CLI/API alone would not preserve distribution, upgrade, product-env, or state-root behavior.
7. **Security changes:** Tightening paths, approvals or tool permissions is desirable but can be behavior-breaking. Record intentional changes and keep compatibility modes explicit.
8. **API uncertainty:** FastAPI routes are dependency-generated. Capture actual route schemas/status/stream behavior from the currently pinned runtime before declaring compatible endpoints.
9. **No Git metadata:** This workspace copy has no `.git` directory, so revision-based history and in-place behavior recording tools that require Git may be unavailable here.

## 13. Feature compatibility checklist

Status meanings follow the migration directive. “Analyzed” means source-level behavior has been identified; it does not mean Rust support exists.

| Feature | Original implementation | Rust status | Evidence / next validation |
|---|---|---|---|
| Eight-agent roster and factory construction | `swarm.py`, agent factories | Not analyzed for exact behavior capture | Record prompts, model configs and factory defaults |
| Orchestrator routing | Prompt + Agency Swarm handoff/message graph | Not analyzed for behavioral parity | Capture handoff and parallel-message scenarios |
| Email/calendar/Slack and Composio actions | `virtual_assistant/`, `shared_tools/`, `helpers.py` | Not analyzed for API parity | Inventory action schemas, auth, effects and confirmations |
| Web/scholarly research | `deep_research/`, `ScholarSearch.py` | Not analyzed for parity | Record HTTP inputs/results, citations, failures |
| Data analysis and persistent Python | `data_analyst_agent/`, Agency Swarm tools | Not analyzed; high migration risk | Decide Rust-native analysis vs isolated Python compatibility |
| Document creation/edit/convert/restore | `docs_agent/` | Not analyzed for artifact parity | Golden files and platform-specific conversion tests |
| Slides authoring/validation/export | `slides_agent/` | Not analyzed for artifact parity | PPTX/rendered output corpus and dependency strategy |
| Image/video/audio features | media agent packages | Not analyzed for provider parity | Provider capability matrix, sample artifacts, cost/limits |
| npm launcher/platform binaries | `bin/`, `scripts/`, npm packages | Not analyzed for distribution parity | Platform matrix, argv/env/output capture |
| Python CLI/bootstrap | `run_utils.py` | Not analyzed for operational parity | OS-specific clean-install/recovery capture |
| HTTP API | `server.py` + Agency Swarm integration | Not analyzed; route contract delegated | Capture OpenAPI, auth, streaming and error contracts |
| Local files and generated artifacts | Tools, `mnt`, `uploads` | Not analyzed for filesystem parity | Path containment, retention and output-format tests |
| Tracing/logging and shutdown | Agency Swarm SDK + setup | Not analyzed | Capture events, redaction and cancellation/shutdown behavior |
| Docker and release workflows | Docker/compose, GitHub Actions, npm publishing | Not analyzed | Build and release matrix; non-root/health/security review |

## Audit boundary

This report reflects repository documentation, static source inspection, the `rust-anything analyze --whole-app --json` inventory, and module-level audits. It does not claim that the project was installed, launched, tested, benchmarked, or security-tested. Exact external-framework behavior and provider compatibility remain unverified until characterized against a runnable original installation.
