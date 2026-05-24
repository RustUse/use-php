# use-php-syntax

PHP syntax label primitives for `RustUse`.

## Experimental

`use-php-syntax` is experimental while `use-php` remains below `0.3.0`.

## Example

```rust
use use_php_syntax::{PhpDeclarationKind, PhpKeyword, PhpModifier, is_php_keyword};

assert!(is_php_keyword("class"));
assert_eq!(PhpKeyword::Readonly.as_str(), "readonly");
assert_eq!(PhpDeclarationKind::Trait.to_string(), "trait");
assert_eq!("final".parse::<PhpModifier>()?, PhpModifier::Final);
# Ok::<(), use_php_syntax::PhpSyntaxError>(())
```

## Scope

- PHP keyword, visibility, declaration, modifier, and control-flow labels.

## Non-goals

- PHP parsing, syntax trees, formatting, linting, or grammar validation.

## License

Licensed under either Apache-2.0 or MIT.
