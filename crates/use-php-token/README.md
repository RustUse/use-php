# use-php-token

PHP token metadata primitives for `RustUse`.

## Experimental

`use-php-token` is experimental while `use-php` remains below `0.3.0`.

## Example

```rust
use use_php_token::{PhpToken, PhpTokenCategory, PhpTokenSpan, PhpTokenText};

let token = PhpToken::new(
    PhpTokenCategory::Identifier,
    PhpTokenText::new("$value")?,
    PhpTokenSpan::new(4, 10)?,
);

assert_eq!(token.category(), PhpTokenCategory::Identifier);
assert_eq!(token.text().as_str(), "$value");
# Ok::<(), use_php_token::PhpTokenError>(())
```

## Scope

- Token categories, delimiter labels, comment labels, operators, literals, and identifiers.
- Lightweight token text and span metadata.

## Non-goals

- A PHP lexer, tokenizer, parser, or grammar implementation.

## License

Licensed under either Apache-2.0 or MIT.
