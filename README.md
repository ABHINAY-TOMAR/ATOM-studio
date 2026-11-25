# ATOM Studio

ATOM Studio is the desktop evolution of OpenSwarm. This repository preserves the original OpenSwarm implementation as a reference while establishing a separate Rust-powered desktop foundation.

**Current status:** the Tauri 2 desktop shell is implemented and verified locally on Windows x64. It includes a React/TypeScript workspace view, Appearance preferences, and a tested frontend-to-Rust IPC connection. The existing OpenSwarm agents and workflows have not yet been migrated into the desktop app.

## Scope

The current development focus is migrating existing OpenSwarm functionality into a secure, maintainable desktop application. The current shell is intentionally limited to Workspace and Appearance; it does not claim to provide connected agents, providers, integrations, sessions, or deliverable workflows.

The original OpenSwarm source remains in this repository as reference. Future creative studios and integrations are outside the current scope.

## Desktop development

### Prerequisites

- Rust and Cargo
- Node.js and npm
- Tauri desktop prerequisites for your operating system
- On Windows, WebView2 Runtime and the Microsoft C++ Build Tools/Windows SDK for the standard MSVC workflow

The current Windows development environment used the Rust GNU toolchain with MinGW because MSVC build prerequisites were not detected. Do not treat this as verification of MSVC, installer, or release builds.

### Run the application

From `rust/apps/desktop`:

```sh
npm ci
npm run tauri dev
```

### Verify

From `rust/apps/desktop`:

```sh
npm run build
npm run verify:ipc
```

The IPC verifier requires the Rust executable to have been built. From `rust/`:

```sh
cargo fmt --all -- --check
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace
```

See [`rust/apps/desktop/README.md`](rust/apps/desktop/README.md) for desktop-specific instructions and [`docs/DESKTOP_BUILD_SPIKE.md`](docs/DESKTOP_BUILD_SPIKE.md) for the observed environment, verification details, known issues, and limitations.

## Architecture and migration documents

- [Migration blueprint](docs/ATOM_STUDIO_MIGRATION_BLUEPRINT.md)
- [Migration roadmap](docs/MIGRATION_ROADMAP.md)
- [Rust architecture](docs/RUST_ARCHITECTURE.md)
- [Desktop architecture](docs/DESKTOP_ARCHITECTURE.md)
- [Feature inventory](docs/OPENSWARM_FEATURE_INVENTORY.md)
- [Feature migration matrix](docs/FEATURE_MIGRATION_MATRIX.md)
- [Security threat model](docs/SECURITY_THREAT_MODEL.md)

These documents describe the planned migration; they are not evidence that the planned capabilities have already been implemented.

## Contributing and repository workflow

See the [developer workflow](docs/DEVELOPMENT_WORKFLOW.md), [branch and commit conventions](docs/COMMIT_CONVENTION.md), [repository setup checklist](docs/GITHUB_REPOSITORY_SETUP.md), [release process](docs/RELEASE_PROCESS.md), and [security policy](SECURITY.md). Changes use issues, short-lived branches, pull requests, independent review, and Windows desktop CI.

See [CHANGELOG.md](CHANGELOG.md) for merged project changes.

## Known limitations

- The GNU Windows build emits a linker warning: `.rsrc merge failure: multiple non-default manifests`. The local build and IPC verification passed, but the warning remains unresolved and must be addressed before packaging.
- MSVC builds, installers, signing, release packaging, and other operating systems have not been verified.
- Tauri bundling is disabled for the current spike.
- The desktop UI has no migrated OpenSwarm runtime functionality yet.

Refer to [`docs/DESKTOP_BUILD_SPIKE.md`](docs/DESKTOP_BUILD_SPIKE.md) for the complete evidence and limitations. This repository is licensed under MIT; see [`LICENSE`](LICENSE).
