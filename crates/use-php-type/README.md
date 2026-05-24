# use-php-type

PHP type metadata primitives for `RustUse`.

## Experimental

`use-php-type` is experimental while `use-php` remains below `0.3.0`.

## Example

```rust
use use_php_type::{PhpScalarType, PhpType, PhpTypeName};

let dto = PhpType::named(PhpTypeName::new("App\\Dto\\UserData")?);
let union = PhpType::union(vec![PhpType::scalar(PhpScalarType::String), dto])?;

assert_eq!(union.kind().as_str(), "union");
assert_eq!(union.to_string(), "string|App\\Dto\\UserData");
# Ok::<(), use_php_type::PhpTypeError>(())
```

## Scope

- Scalar, nullable, union, intersection, mixed, never, void, callable, iterable, object, and class-like type labels.

## Non-goals

- Type checking, variance, generics, PHPStan/Psalm parsing, or runtime reflection.

## License

Licensed under either Apache-2.0 or MIT.
