# ATOM Studio desktop build spike

This is the initial Tauri 2 + Rust + React/TypeScript + Vite desktop shell. The Tauri adapter depends on the separate `atom-core` Rust library. It is deliberately separate from the OpenSwarm source tree and does not include agents or migrated OpenSwarm behavior.

## Development

From `rust/apps/desktop`:

```sh
npm install
npm run tauri dev
```

This starts the Vite development server and Tauri desktop window. Build the frontend alone with `npm run build`. Run the Rust workspace checks from `rust/` with `cargo check --workspace` and `cargo test --workspace`.

## Frontend-to-Rust smoke test

After `cargo build --workspace` from `rust/` (and `npm install`), run:

```sh
npm run verify:ipc
```

The smoke test starts Vite and the built desktop process, clicks the UI's Rust connection check through a temporary local WebView2 DevTools endpoint, verifies the `atom-core` response returns through Tauri and appears in the React UI, then closes both processes. It is a development-only verification helper; do not expose remote debugging in a release build.

## Environment notes

The initial spike was built and launched on Windows 10 x64 with Rust/Cargo 1.100.0, Node 26.13.1, npm 11.20.0 and WebView2 Runtime 158.0.4370.33. This environment has the Rust GNU toolchain and MinGW-w64; Tauri's CLI does not detect the Microsoft Visual Studio C++/Windows SDK prerequisites for the supported MSVC workflow. See `../../../docs/DESKTOP_BUILD_SPIKE.md` for exact verified commands, warnings and remaining platform limitations.

The Tauri bundler is disabled for this spike. Installer creation, signing, update configuration, other operating systems, and release support are not verified.
