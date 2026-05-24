# use-composer-json

Composer JSON metadata primitives for `RustUse`.

## Experimental

`use-composer-json` is experimental while `use-php` remains below `0.3.0`.

## Example

```rust
use use_composer_json::{ComposerJson, ComposerPackageName, ComposerRequirement};

let package = ComposerJson::new()
    .with_name(ComposerPackageName::new("acme/demo")?)
    .with_requirement("php", ComposerRequirement::new("^8.2")?);

assert_eq!(package.name().unwrap().vendor(), "acme");
assert!(package.requirements().contains_key("php"));
# Ok::<(), use_composer_json::ComposerJsonError>(())
```

## Scope

- Composer package names, requirements, scripts, repositories, autoload config metadata, stability labels, and package types.

## Non-goals

- Dependency resolution, lockfile parsing, plugin execution, repository fetching, or package installation.

## License

Licensed under either Apache-2.0 or MIT.
