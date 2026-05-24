# use-php-symbol

PHP symbol metadata primitives for `RustUse`.

## Experimental

`use-php-symbol` is experimental while `use-php` remains below `0.3.0`.

## Example

```rust
use use_php_symbol::{PhpSymbol, SymbolKind, SymbolName};

let symbol = PhpSymbol::new(SymbolKind::Class, SymbolName::new("ExampleController")?);

assert_eq!(symbol.kind(), SymbolKind::Class);
assert_eq!(symbol.name().as_str(), "ExampleController");
# Ok::<(), use_php_symbol::PhpSymbolError>(())
```

## Scope

- Class, interface, trait, enum, function, constant, method, property, and parameter symbol metadata.
- Lightweight name validation helpers.

## Non-goals

- Symbol tables, name resolution, reflection, autoloading, or static analysis engines.

## License

Licensed under either Apache-2.0 or MIT.
