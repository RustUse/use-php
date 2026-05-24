# use-php

Feature-gated facade crate for `RustUse` PHP language, package, standards, and ecosystem primitives.

## Experimental

`use-php` is experimental while the release line remains below `0.3.0`.

## Example

```rust
#[cfg(feature = "full")]
{
	use use_php::prelude::{
		ComposerJson, ComposerPackageName, PhpVersion, WordPressPostTypeSlug,
	};

	let version: PhpVersion = "8.3.2".parse()?;
	let package = ComposerJson::new().with_name(ComposerPackageName::new("acme/demo")?);
	let post_type = WordPressPostTypeSlug::new("book")?;

	assert_eq!(version.major(), 8);
	assert_eq!(package.name().unwrap().package(), "demo");
	assert_eq!(post_type.as_str(), "book");
}
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Scope

- One facade dependency for focused PHP primitive crates.
- Feature-gated modules that mirror the child crate boundaries.
- Metadata primitives for language labels, Composer, Packagist, PSR, extensions, WordPress, Drupal, and Laravel.

## Non-goals

- PHP interpreter behavior, full parser frameworks, Composer resolution, network clients, or framework SDKs.

## License

Licensed under either Apache-2.0 or MIT.
