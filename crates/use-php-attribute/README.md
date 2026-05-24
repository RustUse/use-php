# use-php-attribute

PHP attribute metadata primitives for `RustUse`.

## Experimental

`use-php-attribute` is experimental while `use-php` remains below `0.3.0`.

## Example

```rust
use use_php_attribute::{PhpAttributeName, PhpAttributeReference, PhpAttributeTarget};

let attribute = PhpAttributeReference::new(PhpAttributeName::new("App\\Route")?)
    .with_target(PhpAttributeTarget::Method);

assert_eq!(attribute.name().as_str(), "App\\Route");
assert_eq!(attribute.targets(), &[PhpAttributeTarget::Method]);
# Ok::<(), use_php_attribute::PhpAttributeError>(())
```

## Scope

- Attribute names, targets, simple argument metadata, and repeatability flags.

## Non-goals

- Attribute parsing, reflection, evaluation, or framework routing behavior.

## License

Licensed under either Apache-2.0 or MIT.
