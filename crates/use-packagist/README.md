# use-packagist

Packagist metadata primitives for `RustUse`.

## Experimental

`use-packagist` is experimental while `use-php` remains below `0.3.0`.

## Example

```rust
use use_packagist::{PackagistDownloadCount, PackagistPackageName, PackagistStability};

let package = PackagistPackageName::new("symfony/console")?;
let downloads = PackagistDownloadCount::new(42);

assert_eq!(package.vendor(), "symfony");
assert_eq!(downloads.get(), 42);
assert_eq!(PackagistStability::Stable.as_str(), "stable");
# Ok::<(), use_packagist::PackagistError>(())
```

## Scope

- Packagist package identifiers, labels, package types, stability labels, and simple statistic wrappers.

## Non-goals

- Network clients, API pagination, package search, or Composer resolution.

## License

Licensed under either Apache-2.0 or MIT.
