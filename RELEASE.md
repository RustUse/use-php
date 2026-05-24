# Release Policy

RustUse/use-php is not published yet. The root workspace metadata keeps `publish = false` as the default, while the current first-wave crate manifests opt in with `publish = true`.

## First Publish Wave

The intended first publish candidates are every focused crate under `crates/` plus `use-php`.

Publish all focused crates first. Wait for crates.io index propagation, then publish `use-php`.

## Publish Surface

Before the first publish wave, confirm that the release surface:

- keeps the workspace-level default at `publish = false`
- keeps every focused crate under `crates/` at `publish = true`
- keeps `crates/use-php/Cargo.toml` at `publish = true`

## Versioning

- The workspace currently uses lockstep `0.x.y` versioning.
- Before `1.0`, breaking changes should bump the minor version.
- Before `1.0`, additive compatible changes should bump the patch version.
- The workspace should remain below `0.3.0` until production readiness is intentional.

## Automated Release Validation

The repository includes a dedicated release-validation path:

- `.github/workflows/publish-readiness.yml` runs on pull requests, pushes to `main`, and manual dispatch.
- `.github/workflows/facade-publish-readiness.yml` is a manual post-publication check that dry-runs `use-php` only after the focused crates are live on crates.io.

## Version and Changelog Automation

The repository includes `release-plz` configuration in `release-plz.toml` and maintainer workflows under `.github/workflows/release-plz-*.yml`.

- `Release PR Automation` opens or updates a release PR with lockstep version changes for every publishable crate in the workspace.
- The workspace is configured with one `version_group` so all published crates keep the same version.
- The root `CHANGELOG.md` remains the shared changelog and is updated through the `use-php` package entry.

## Publish Readiness Checklist

1. Confirm `cargo fmt` is clean.
2. Confirm `cargo check --workspace --all-features` passes.
3. Confirm `cargo test --workspace --all-features` passes.
4. Confirm `cargo test --workspace --no-default-features` passes.
5. Confirm `cargo clippy --workspace --all-targets --all-features` passes.
6. Confirm `cargo deny check` and `cargo audit` pass when the tools are installed.
7. Review README examples, crate metadata, `Cargo.lock`, and changelog entries.
8. Confirm the focused crates are available on crates.io before attempting the facade publish.
