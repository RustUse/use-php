# Contributing

Thank you for helping improve RustUse/use-php.

This workspace is for small, composable PHP language, package, standards, and ecosystem primitives. Keep contributions focused on metadata, validation helpers, and lightweight data structures. Do not add PHP interpreter behavior, full parser frameworks, package resolvers, network clients, or framework SDK integrations.

## Development

Run the core validation suite before opening a pull request:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo test --workspace --no-default-features
```

Prefer adding focused tests near the primitive APIs they cover. Keep dependencies out of focused crates unless there is a clear RustUse-wide reason to add one.
