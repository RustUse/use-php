# use-psr

PHP-FIG PSR metadata primitives for `RustUse`.

## Experimental

`use-psr` is experimental while `use-php` remains below `0.3.0`.

## Example

```rust
use use_psr::{PsrCategory, PsrMetadata, PsrNumber, PsrStatus, PsrTitle};

let metadata = PsrMetadata::new(
    PsrNumber::new(4)?,
    PsrTitle::new("Autoloading Standard")?,
    PsrStatus::Accepted,
    PsrCategory::Autoloading,
);

assert_eq!(metadata.identifier(), "PSR-4");
# Ok::<(), use_psr::PsrError>(())
```

## Scope

- PSR numbers, titles, status labels, and broad categories.

## Non-goals

- Implementing PSR standards, HTTP messages, logging, cache, container, or event behavior.

## License

Licensed under either Apache-2.0 or MIT.
