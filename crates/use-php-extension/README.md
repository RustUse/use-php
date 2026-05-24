# use-php-extension

PHP extension metadata primitives for `RustUse`.

## Experimental

`use-php-extension` is experimental while `use-php` remains below `0.3.0`.

## Example

```rust
use use_php_extension::{PhpExtensionKind, PhpExtensionName, PhpExtensionRequirement};

let requirement = PhpExtensionRequirement::required(PhpExtensionName::new("mbstring")?)
    .with_kind(PhpExtensionKind::Bundled);

assert_eq!(requirement.name().as_str(), "mbstring");
assert!(requirement.is_required());
# Ok::<(), use_php_extension::PhpExtensionError>(())
```

## Scope

- Extension names, requirement flags, extension kinds, and simple version constraint labels.

## Non-goals

- Extension loading, runtime inspection, PECL clients, or platform compatibility checks.

## License

Licensed under either Apache-2.0 or MIT.
