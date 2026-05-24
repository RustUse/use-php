# Releasing

This repository uses a focused-first release flow rather than a single-crate manual publish pattern.

## Current release state

`use-php` publishes a first wave of focused crates before the `use-php` facade crate.

## Canonical release guide

Use [RELEASE.md](RELEASE.md) as the authoritative release policy for first-wave publish scope, focused-crate publish ordering, publish readiness checks, and trusted publishing setup after the first public wave.

## Current automation

The repository includes the specialized workflows that match this release shape:

- `publish-readiness.yml`
- `facade-publish-readiness.yml`
- `release-plz-pr.yml`
- `release-plz-release.yml`
