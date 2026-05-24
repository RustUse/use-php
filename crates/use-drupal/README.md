# use-drupal

Drupal metadata primitives for `RustUse`.

## Experimental

`use-drupal` is experimental while `use-php` remains below `0.3.0`.

## Example

```rust
use use_drupal::{DrupalModuleName, DrupalPermission, DrupalRouteName};

let module = DrupalModuleName::new("book_tools")?;
let route = DrupalRouteName::new("book_tools.index")?;
let permission = DrupalPermission::new("administer book tools")?;

assert_eq!(module.as_str(), "book_tools");
assert_eq!(route.as_str(), "book_tools.index");
assert_eq!(permission.as_str(), "administer book tools");
# Ok::<(), use_drupal::DrupalError>(())
```

## Scope

- Module names, theme names, route names, entity type IDs, config object names, permission strings, and extension kind labels.

## Non-goals

- Drupal API clients, service containers, routing, plugins, config storage, or runtime integration.

## License

Licensed under either Apache-2.0 or MIT.
