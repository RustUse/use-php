# use-laravel

Laravel metadata primitives for `RustUse`.

## Experimental

`use-laravel` is experimental while `use-php` remains below `0.3.0`.

## Example

```rust
use use_laravel::{ArtisanCommandName, LaravelConfigKey, LaravelRouteName};

let route = LaravelRouteName::new("books.index")?;
let command = ArtisanCommandName::new("books:sync")?;
let config = LaravelConfigKey::new("cache.default")?;

assert_eq!(route.as_str(), "books.index");
assert_eq!(command.as_str(), "books:sync");
assert_eq!(config.as_str(), "cache.default");
# Ok::<(), use_laravel::LaravelError>(())
```

## Scope

- Route names, middleware names, Artisan command names, migration names, service provider names, config keys, and metadata references.

## Non-goals

- Laravel runtime behavior, application containers, routing, migrations, queues, or framework integration.

## License

Licensed under either Apache-2.0 or MIT.
