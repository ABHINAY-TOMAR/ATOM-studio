# ATOM Studio Desktop Build Spike

**Status:** Implemented and verified on the local Windows x64 development machine. This is a desktop foundation only; OpenSwarm agent/runtime features have not been migrated.

## What was built

- Tauri 2 Rust desktop application in `rust/apps/desktop/src-tauri/`, in a Cargo workspace with a small, separate `rust/crates/atom-core/` library.
- React 18 + TypeScript + Vite frontend with Tailwind CSS and Lucide icons in `rust/apps/desktop/`.
- Dark-first ATOM Studio shell with original branding, a focused Workspace view, Appearance settings, responsive sidebar navigation, foundation overview and Rust connection status panel. Navigation intentionally exposes no unimplemented OpenSwarm workflows.
- Typed narrow Tauri command `ipc_smoke_test` in `src-tauri/src/main.rs`, delegating to the `atom-core` library; the React frontend invokes it through `@tauri-apps/api/core`.
- Minimal capability declaration with no plugin permissions; no OpenSwarm code, agents, tools, providers or user data integration.
- Development verifier starts Vite and the desktop process, exercises the frontend through a local WebView2 DevTools endpoint, checks the Workspace/Appearance navigation, theme and density controls, responsive sidebar, and Rust IPC response, then closes both processes.
- Appearance supports dark/light themes and comfortable/compact density. Preferences are persisted in local storage and fall back safely when storage is unavailable.

## Verified environment

| Component                                   | Observed value                            | Verification                                         |
| ------------------------------------------- | ----------------------------------------- | ---------------------------------------------------- |
| OS                                          | Windows 10.0.19045 x64                    | `tauri info`                                         |
| Rust                                        | rustc 1.100.0                             | `rustc --version` / Tauri info                       |
| Cargo                                       | 1.100.0                                   | `cargo --version` / Tauri info                       |
| Active Rust toolchain                       | `stable-x86_64-pc-windows-gnu`            | `rustup show`                                        |
| Alternative Rust toolchain                  | `stable-x86_64-pc-windows-msvc` installed | `rustup toolchain list`                              |
| Node.js                                     | v26.13.1                                  | `node --version`                                     |
| npm                                         | 11.20.0                                   | `npm --version`                                      |
| WebView2 Runtime                            | 158.0.4370.33                             | Registry, installed runtime directory, `tauri info`  |
| MinGW-w64                                   | Available via WinLibs                     | `gcc`, `g++`, `windres` located                      |
| Visual Studio C++ Build Tools / Windows SDK | Not detected                              | `tauri info`; no MSVC compiler/SDK in standard paths |

This verifies one Windows development configuration only. It does not establish supported customer hardware, MSVC builds, installer/signing, macOS, Linux, or other Windows versions.

## Commands run and results

Successful commands:

```text
npm install --no-audit --no-fund --progress=false --fetch-retries=1 --fetch-timeout=30000
npm ls --depth=0
npm run build
npm run verify:ipc
rustup run stable-x86_64-pc-windows-gnu cargo build --workspace
rustup run stable-x86_64-pc-windows-gnu cargo check --workspace
rustup run stable-x86_64-pc-windows-gnu cargo test --workspace
rustup run stable-x86_64-pc-windows-gnu cargo clippy --workspace --all-targets -- -D warnings
rustup run stable-x86_64-pc-windows-gnu cargo fmt --all --check
```

Results:

- npm dependencies installed and locked in `rust/apps/desktop/package-lock.json`.
- TypeScript typecheck and Vite production bundle passed (1,579 modules transformed).
- Tauri/Rust workspace build and check passed on the GNU target.
- Core unit test passed: `response_includes_requested_name` (1 passed).
- Clippy with warnings denied and `cargo fmt --check` passed.
- Vite dev server returned HTTP 200 and the ATOM Studio document title.
- A native Tauri/WebView2 window was launched from the built desktop executable. The verifier connected to that window's local WebView2 debugging endpoint and verified Workspace/Appearance navigation, persistent theme and density selection, a collapsed 640px emulated viewport sidebar, and the full round trip: **React button → Tauri invoke → Rust command → `atom-core` → rendered React response**: `Hello, ATOM Studio — Rust IPC is working.`
- Tauri CLI `info` recognized the frontend/config and WebView2, but reported the missing Visual Studio C++/Windows SDK prerequisite for the standard MSVC Windows setup.

## Issues encountered and resolved

1. The first npm install timed out, leaving incomplete `node_modules`/lock state. A clean reinstall from a removed, newly-generated lockfile completed; final `npm ls` showed the expected dependency set and the production build passed.
2. Initial MSVC-target Cargo compilation failed because `link.exe` resolved to Git's Unix utility rather than Microsoft's linker. `tauri info` confirmed no Visual Studio C++/Windows SDK installation. No tools were reinstalled. The already-installed GNU Rust toolchain and MinGW-w64 successfully built and launched the spike.
3. The first direct IPC harness run omitted the Vite dev server; the Tauri window therefore displayed a connection error. The verifier was corrected to start Vite before the app and then the end-to-end IPC check passed.
4. The first shell verification exposed that the responsive test emulated a non-mobile device viewport, so WebView2 retained the native window width. The verifier now explicitly enables mobile viewport emulation to test the 640px CSS breakpoint. The full verifier subsequently passed.
5. GNU linking emits a warning: `.rsrc merge failure: multiple non-default manifests`. The executable and test harness still link and run, but the warning is an unresolved toolchain/resource-manifest issue. Resolve it before packaging/release or switch to the standard MSVC toolchain.

## Development commands

From `rust/apps/desktop`:

```sh
npm install
npm run tauri dev
```

Additional checks: `npm run build`, `npm run verify:ipc` (requires a preceding Rust build), and from `rust/`: `cargo fmt --all --check`, `cargo check --workspace`, `cargo test --workspace`, and `cargo clippy --workspace --all-targets -- -D warnings`.

## Limitations

- Tauri bundling is disabled (`bundle.active: false`); no installer or release artifact is built.
- The standard MSVC toolchain and Windows SDK are missing. GNU worked for this local spike, but do not treat that as MSVC compatibility or a supported production Windows build.
- The app's CSP is unset for the build spike. Define and verify a production CSP before integrating external content or releasing.
- Provider configuration, secure credential storage, session persistence, workspaces/file permissions, notifications, accessibility review, and actual OpenSwarm capabilities are not implemented in this phase.
- Workspace overview is shell-level presentation, not a connected product workflow. Appearance preferences are functional and local to this desktop webview.
- Only local desktop window execution and the command round-trip were verified; no automated visual review/screenshot or multi-platform checks were available.

## Next phase recommendation

Review the spike and decide the supported OS/toolchain matrix. Install/use the Visual Studio C++ Build Tools + Windows SDK (or explicitly approve a GNU-based support strategy), address the manifest warning, and define the production CSP and release packaging constraints before implementing the next desktop/core phase. Do not begin OpenSwarm migration until this foundation is accepted.
