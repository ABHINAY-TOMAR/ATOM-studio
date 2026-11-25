# Release process

ATOM Studio has no release artifacts or production support commitment yet. The Tauri bundler is disabled, the Windows GNU build has an unresolved `.rsrc merge failure: multiple non-default manifests` linker warning, and MSVC, installer, signing, and release builds remain unverified. Do not publish an installer or claim platform support until those risks are resolved and tested.

When a release is authorized:

1. Select completed issues and freeze the release scope.
2. Verify local Rust formatting, check, tests, Clippy, and build; verify frontend install/build and the desktop IPC smoke test on the supported environment.
3. Confirm CI is green and audit the complete diff, dependency changes, security findings, and known limitations.
4. Treat `rust/Cargo.toml` (`[workspace.package]`) as the desktop version source of truth. Update the version consistently in `rust/apps/desktop/package.json`, `rust/apps/desktop/package-lock.json` (top-level and `packages[""]` entries), and `rust/apps/desktop/src-tauri/tauri.conf.json`. Run `node scripts/check-desktop-version.mjs`. Do not change the separate legacy OpenSwarm version in root `package.json` or `pyproject.toml` as part of an ATOM Studio desktop release.
5. Move completed `CHANGELOG.md` entries out of `Unreleased` into a dated `X.Y.Z` section, preserve the required headings, and create a fresh `Unreleased` section.
6. Commit the release metadata on a reviewed branch and merge it through the normal PR process.
7. Create an annotated tag on the merged commit: `git tag -a vX.Y.Z -m "ATOM Studio vX.Y.Z"`.
8. Build artifacts only on the explicitly supported target/toolchain. Verify each artifact's identity, target, launch behavior, and checksums; retain build logs.
9. Publish a GitHub Release with notes and attach only artifacts verified in step 8. Clearly list unverified targets and known limitations.
10. Record post-release fixes and follow-up work in issues and the next changelog cycle.

Do not create a release tag or GitHub Release as part of M2. Use Semantic Versioning (`MAJOR.MINOR.PATCH`); during pre-1.0 development use `0.x.0` for milestone/feature progression and `0.x.y` for compatible fixes. Version numbers are deliberate release decisions, not automatic increments.
