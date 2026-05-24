.PHONY: help fmt check lint test test-minimal build doc examples audit deny publish-dry-run-focused publish-dry-run-facade release-readiness facade-post-publish-validation verify

FOCUSED_CRATES := use-php-version use-php-token use-php-syntax use-php-symbol use-php-namespace use-php-type use-php-attribute use-php-docblock use-php-ini use-php-error use-php-autoload use-composer-json use-packagist use-psr use-php-extension use-wordpress use-wordpress-hook use-wordpress-block use-drupal use-laravel
FACADE_CRATE := use-php

help:
	@printf "%s\n" \
		"help                           Show available repository tasks" \
		"fmt                            Check formatting with rustfmt" \
		"check                          Run cargo check for the workspace" \
		"lint                           Run clippy with warnings denied" \
		"test                           Run workspace tests with all features" \
		"test-minimal                   Run workspace tests with no default features" \
		"build                          Build the workspace with all features" \
		"doc                            Build workspace docs without dependencies" \
		"examples                       Check all examples" \
		"audit                          Run cargo-audit" \
		"deny                           Run cargo-deny" \
		"publish-dry-run-focused        List package contents and dry-run publish focused crates" \
		"publish-dry-run-facade         Dry-run publish $(FACADE_CRATE) after crates.io propagation" \
		"release-readiness              Run the pre-release focused-crate validation path" \
		"facade-post-publish-validation Dry-run the facade crate after focused crates are live" \
		"verify                         Run the main workspace validation path"

fmt:
	cargo fmt --all -- --check

check:
	cargo check --workspace --all-features

lint:
	cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
	cargo test --workspace --all-features

test-minimal:
	cargo test --workspace --no-default-features

build:
	cargo build --workspace --all-features

doc:
	cargo doc --workspace --all-features --no-deps

examples:
	cargo check --workspace --all-features --examples

audit:
	cargo audit

deny:
	cargo deny check

publish-dry-run-focused:
	@if [ -z "$(strip $(FOCUSED_CRATES))" ]; then \
		printf "%s\n" "No focused crates configured"; \
	else \
		for crate in $(FOCUSED_CRATES); do \
			cargo package --list -p $$crate; \
			cargo publish --dry-run --allow-dirty -p $$crate; \
		done; \
	fi

publish-dry-run-facade:
	cargo publish --dry-run --allow-dirty -p $(FACADE_CRATE)

release-readiness: verify examples test-minimal publish-dry-run-focused

facade-post-publish-validation: publish-dry-run-facade

verify: fmt lint test build
