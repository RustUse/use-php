# use-php-ini

PHP INI metadata primitives for `RustUse`.

## Experimental

`use-php-ini` is experimental while `use-php` remains below `0.3.0`.

## Example

```rust
use use_php_ini::{PhpIniDirective, PhpIniDirectiveName, PhpIniValue};

let directive = PhpIniDirective::new(
    PhpIniDirectiveName::new("memory_limit")?,
    PhpIniValue::String("128M".to_string()),
);

assert_eq!(directive.to_string(), "memory_limit = 128M");
# Ok::<(), use_php_ini::PhpIniError>(())
```

## Scope

- INI section names, directive names, scalar values, environment labels, and simple line parsing.

## Non-goals

- A complete INI parser, file merger, runtime configuration loader, or SAPI integration.

## License

Licensed under either Apache-2.0 or MIT.
