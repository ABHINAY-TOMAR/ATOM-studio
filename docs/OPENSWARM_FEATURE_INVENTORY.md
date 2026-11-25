# OpenSwarm Feature Inventory

Source basis: `REVERSE_ENGINEERING_REPORT.md`, repository source/configuration, and analyzer summary (142 Python files, ~21,355 LOC, two Python test files). Capabilities documented in agent prompts are intended behavior; actual availability depends on registered tools, provider keys, connected accounts, and framework behavior.

| Capability | Current implementation/evidence | Important boundary |
|---|---|---|
| Agent team | `swarm.py` assembles orchestrator plus virtual assistant, deep research, data analyst, docs, slides, image and video agents | Agency Swarm owns lifecycle/context details |
| Orchestration | Orchestrator prompt; orchestrator-to-specialist `SendMessage`, all distinct pairs `Handoff` (`swarm.py`) | Prompt chooses behavior; graph permits broad handoffs |
| Model selection | `config.py`, `DEFAULT_MODEL`, OpenAI IDs or slash-prefixed LiteLLM model | Provider SDK/model capabilities vary |
| Virtual assistant | `virtual_assistant/` email/calendar/Slack/files and Composio discovery/execution | Account permissions and external side effects |
| Research | `deep_research/`, web search, `ScholarSearch.py` | SearchApi key/network, provider calls, citations depend on tool output |
| Data analysis | `data_analyst_agent/`, persistent shell/IPython, data science dependencies | Python execution is a capability, not a sandbox guarantee |
| Documents | `docs_agent/` create/list/view/edit/restore/convert, HTML preview and file utilities | WeasyPrint, Playwright, DOCX/PDF conversion and filesystem paths |
| Slides | `slides_agent/` HTML/PPTX creation and editing, preview, checks, image search/generation | HTML-to-PPTX and browser/media dependencies; tools are selectively registered |
| Images | `image_generation_agent/` generation/edit/composition/background removal | Provider/key availability; local asset writes and remote calls |
| Video/audio | `video_generation_agent/` generate/edit/trim/combine, audio/subtitles, image operations | Provider and FFmpeg/codec availability; expensive/resource-heavy |
| Shared tools | `shared_tools/` Composio actions/discovery/connections, file copy/model availability | Must have explicit per-agent capabilities and audit in replacement |
| File handling | `mnt`, uploads and agent-specific tools | Current access patterns vary; global path policy not demonstrated |
| Context/history | Agency Swarm threads/callbacks, persistent interpreters in some agents | Persistence schema/retention is partly framework-owned |
| CLI/bootstrap | `bin/openswarm`, `run_utils.py`, `scripts/` platform binary selection/bootstrap | Installs runtimes/deps and platform-specific binaries |
| API/server | `server.py` delegates to Agency Swarm FastAPI integration, port 8080, `./uploads` | Exact routes/auth/streaming need runtime contract capture |
| Packaging/deployment | npm optional platform packages, Python packages, Docker/Compose, GitHub Actions | Cross-platform, release and native dependency burden |

### Key evidence and gaps

- `swarm.py`: eight-agent roster and communication wiring.
- `server.py`: framework-delegated FastAPI setup; route contract not defined locally.
- `openswarm.config.mjs`: product identity, addons and state-root behavior; branding currently says OpenSwarm.
- `requirements.txt`, `pyproject.toml`, `package.json`, `Dockerfile`: runtimes and packaging.
- `tests/`: currently two Python test files, supplemented by smoke/bootstrap scripts and CI.
- Analyzer found 30 migration units (12 ready, 17 caution, 1 blocked); high coupling means feature migration cannot be assumed independent.

Unknown until captured: exact provider/model support by operation, complete tool schemas/effects, API routes and auth, cancellation semantics, thread storage, resource limits, error contracts, and supported hardware/OS matrix.
