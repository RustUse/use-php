# use-php-error

PHP error and diagnostic metadata primitives for `RustUse`.

## Experimental

`use-php-error` is experimental while `use-php` remains below `0.3.0`.

## Example

```rust
use use_php_error::{DiagnosticMessage, PhpDiagnostic, PhpErrorKind, PhpSeverity};

let diagnostic = PhpDiagnostic::new(
    PhpErrorKind::Runtime,
    PhpSeverity::Error,
    DiagnosticMessage::new("Undefined variable")?,
);

assert_eq!(diagnostic.severity(), PhpSeverity::Error);
# Ok::<(), use_php_error::PhpDiagnosticError>(())
```

## Scope

- PHP error levels, diagnostic categories, severity mapping, and diagnostic message/source metadata.

## Non-goals

- Runtime error handling, stack traces, logging, or analyzer rule engines.

## License

Licensed under either Apache-2.0 or MIT.
