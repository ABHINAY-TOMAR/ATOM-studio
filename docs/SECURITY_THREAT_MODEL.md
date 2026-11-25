# ATOM Studio Security Threat Model

Status: design-level threat model based on OpenSwarm source audit; no penetration test or sandbox verification performed.

## Assets and trust boundaries

**Assets:** provider/Composio credentials, conversation and generated data, workspace files, connected-service accounts, local session DB, provider usage/quota, application binaries/update keys.

**Boundaries:** untrusted prompt/tool output ↔ orchestrator; React WebView ↔ Tauri commands; core ↔ providers/Composio; core ↔ workspace filesystem; core ↔ shell/Python/media child processes; app ↔ update/package service.

## Threats and required controls

| Threat | OpenSwarm evidence/risk | ATOM Studio controls and proof |
|---|---|---|
| Prompt injection/tool misuse | Prompt policies and broad registered tools; every agent handoff can reach broad graph | Tool capability allowlist per agent/task; validate arguments; confirmations enforced in core; adversarial integration tests |
| Shell/Python code execution | PersistentShellTool and IPythonInterpreter registered in several agents | Disabled by default until OS-level isolation exists; child-process boundary, restricted filesystem/network, CPU/memory/time limits, cancellation and audit. No claim of sandbox until demonstrated |
| Path traversal/arbitrary file access | File operations, uploads, document project/name paths, CopyFile destination | Workspace grants, canonical path checks, symlink policy, deny traversal/out-of-root, safe overwrite/atomic write; tests on Windows path forms |
| Credential leakage | Environment/state keys, provider/tool errors and tracing | OS credential store, secret redaction in tracing/errors/events, UI never receives secret values, no keys in crash reports; automated leak tests |
| SSRF/network abuse | Search and media tooling can consume remote/caller URLs | URL parser/allowlist, block loopback/private/link-local/metadata IPs as policy demands, validate every redirect/DNS resolution, TLS verification, timeout/size limits; integration tests |
| Unauthorized connected-service mutation | Composio can execute arbitrary named actions under user account | Explicit connection and action grants, typed allowlisted schemas, per-action risk classification, confirm send/delete/write, auditable result; test denied paths |
| Resource exhaustion | Parallel agents, persistent interpreters, browser/converters and media operations | Bounded queues/concurrency, per-task deadlines, response/context/file limits, disk quotas, process resource limits, cancellation; stress tests |
| API/IPC privilege escalation | Existing API delegated to framework; desktop introduces WebView IPC | Tauri capability allowlist, narrow commands, input validation, no generic shell/path command, local API disabled unless authenticated and needed; IPC fuzz/integration tests |
| Malicious tool/provider output | External text/files may contain prompt-like data or malformed payload | Treat output as data, schema/size validation, safe rendering, no implicit tool chaining based on output, file type handling and limits |
| Persistence tampering/corruption | No first-party DB identified; proposed SQLite introduces local state | Migrations, transactions, schema validation, restrictive file permissions, backups/recovery; no secrets in DB |
| Supply-chain compromise | Python/npm/native/optional platform dependency surface | Cargo/npm lockfiles, provenance/signing, advisories/license checks, pinned build actions, minimal packaged engines, update signature verification |
| Update compromise | New desktop distribution adds updater risk | Signed releases, verified update metadata, rollback strategy; disable updater until lifecycle/signing validated |

## Prompt-injection boundary

System instructions guide behavior only. Security decisions are code-owned: tool authorization, filesystem scopes, network policy, approval state and resource budget are checked independently of model output. Tool output must not grant permissions or override policy.

## Minimum release gate

No release may claim that shell/Python is sandboxed without a tested isolation mechanism and threat review. Before enabling a capability, run its path/permission/SSRF/resource/secret regression suite, document residual risks and make its scope visible in settings. Any security behavior differing from OpenSwarm must appear in the compatibility matrix.
