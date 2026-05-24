# use-php-namespace

PHP namespace metadata primitives for `RustUse`.

## Experimental

`use-php-namespace` is experimental while `use-php` remains below `0.3.0`.

## Example

```rust
use use_php_namespace::{PhpFullyQualifiedName, PhpNamespaceAlias, PhpNamespacePath};

let namespace = PhpNamespacePath::new("App\\Http\\Controller")?;
let name = PhpFullyQualifiedName::new("\\App\\Http\\Controller\\HomeController")?;
let alias = PhpNamespaceAlias::new("HomeController")?;

assert_eq!(namespace.segments(), vec!["App", "Http", "Controller"]);
assert_eq!(name.to_string(), "\\App\\Http\\Controller\\HomeController");
assert_eq!(alias.as_str(), "HomeController");
# Ok::<(), use_php_namespace::PhpNamespaceError>(())
```

## Scope

- Namespace paths, fully qualified names, relative names, aliases/imports, and global namespace metadata.

## Non-goals

- Name resolution, autoloading, filesystem mapping, or symbol indexing.

## License

Licensed under either Apache-2.0 or MIT.
