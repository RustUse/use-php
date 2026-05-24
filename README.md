# RustUse use-php

Composable PHP language, package, standards, and ecosystem primitives for Rust.

`use-php` is a RustUse facade set for PHP language, package, standards, and ecosystem primitives. It provides small reusable primitives useful for tooling, documentation, metadata validation, static analysis helpers, test fixtures, code generation, and system modeling.

## Experimental

Every crate in this workspace is experimental while the release line remains below `0.3.0`. Expect incremental API cleanup as the first wave settles.

## Scope

- Small focused crates with primitive data types and helpers.
- Rust 2024 APIs with few or no dependencies.
- PHP language labels, package metadata, standards metadata, extension metadata, and framework-adjacent identifiers.
- Facade composition through `use-php`, with implementation living in focused child crates.

## Non-goals

- `use-php` is not a PHP interpreter.
- `use-php` is not a PHP parser framework.
- `use-php` is not a Composer resolver.
- `use-php` is not a WordPress, Drupal, or Laravel SDK.
- `use-php` does not provide network clients, runtime callbacks, package installation, routing, plugin systems, or framework integrations.

## Crate List

| Crate                 | Purpose                                                                                                                 |
| --------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| `use-php`             | Feature-gated facade crate for the full PHP primitive set                                                               |
| `use-php-version`     | PHP version structs, branch labels, comparison primitives, and support phase labels                                     |
| `use-php-token`       | Token category, delimiter, comment, operator, literal, identifier, text, and span metadata                              |
| `use-php-syntax`      | Keyword, visibility, declaration, modifier, and control-flow labels                                                     |
| `use-php-symbol`      | Symbol names and kinds for classes, interfaces, traits, enums, functions, constants, members, and parameters            |
| `use-php-namespace`   | Namespace paths, fully qualified names, relative names, aliases, imports, and global namespace metadata                 |
| `use-php-type`        | Scalar, nullable, union, intersection, mixed, never, void, callable, iterable, object, and class-like type primitives   |
| `use-php-attribute`   | Attribute names, targets, simple arguments, repeatability, and references                                               |
| `use-php-docblock`    | PHPDoc block metadata, summary/body helpers, common tags, tag names, and type strings                                   |
| `use-php-ini`         | INI section names, directive names, scalar values, environments, and simple directive line helpers                      |
| `use-php-error`       | Error levels, diagnostic categories, severity mapping, and diagnostic metadata                                          |
| `use-php-autoload`    | PSR-4 mappings, classmap entries, files entries, and autoload strategy labels                                           |
| `use-composer-json`   | Composer package metadata, requirements, scripts, repositories, autoload config, stability, and package type primitives |
| `use-packagist`       | Packagist package identifiers, metadata labels, download counts, stability, and package type primitives                 |
| `use-psr`             | PHP-FIG PSR numbers, titles, statuses, and categories                                                                   |
| `use-php-extension`   | PHP extension names, requirement metadata, required/optional flags, kind labels, and version constraints                |
| `use-wordpress`       | WordPress plugin headers, theme fields, post types, taxonomies, capabilities, and REST labels                           |
| `use-wordpress-hook`  | WordPress action/filter hook names, priorities, callback metadata, and hook references                                  |
| `use-wordpress-block` | WordPress `block.json`-oriented names, categories, attributes, supports, and asset path labels                          |
| `use-drupal`          | Drupal module names, theme names, routes, entity type IDs, config object names, and permission strings                  |
| `use-laravel`         | Laravel route names, middleware names, Artisan commands, migrations, service providers, and config keys                 |

## Project Structure

```text
use-php/
├── Cargo.toml
├── README.md
├── crates/
│   ├── use-php/
│   ├── use-php-version/
│   ├── use-php-token/
│   ├── use-php-syntax/
│   ├── use-php-symbol/
│   ├── use-php-namespace/
│   ├── use-php-type/
│   ├── use-php-attribute/
│   ├── use-php-docblock/
│   ├── use-php-ini/
│   ├── use-php-error/
│   ├── use-php-autoload/
│   ├── use-composer-json/
│   ├── use-packagist/
│   ├── use-psr/
│   ├── use-php-extension/
│   ├── use-wordpress/
│   ├── use-wordpress-hook/
│   ├── use-wordpress-block/
│   ├── use-drupal/
│   └── use-laravel/
└── .github/
    └── workflows/
```

## Installation

Before the first crates.io release, depend on a Git revision:

```toml
[dependencies]
use-php = { git = "https://github.com/RustUse/use-php", rev = "<commit>" }
```

After publication, use the facade when one dependency is more convenient:

```toml
[dependencies]
use-php = "0.0.1"
```

Or choose focused crates directly:

```toml
[dependencies]
use-php-version = "0.0.1"
use-composer-json = "0.0.1"
use-wordpress-hook = "0.0.1"
```

## Example Usage

```rust
use use_php::prelude::{ComposerJson, ComposerPackageName, PhpVersion, WordPressPostTypeSlug};

let version: PhpVersion = "8.3.2".parse()?;
let package = ComposerJson::new().with_name(ComposerPackageName::new("acme/demo")?);
let post_type = WordPressPostTypeSlug::new("book")?;

assert_eq!(version.major(), 8);
assert_eq!(package.name().unwrap().vendor(), "acme");
assert_eq!(post_type.as_str(), "book");
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Feature Model

The facade defaults to `full`, which enables every focused crate. Consumers can disable default features and opt into a smaller surface:

```toml
[dependencies]
use-php = { version = "0.0.1", default-features = false, features = ["version", "composer-json"] }
```

## Development

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo test --workspace --no-default-features
```

## License

Licensed under either of the following, at your option:

- Apache License, Version 2.0
- MIT license
