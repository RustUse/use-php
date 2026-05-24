# use-php-autoload

PHP autoload metadata primitives for `RustUse`.

## Experimental

`use-php-autoload` is experimental while `use-php` remains below `0.3.0`.

## Example

```rust
use use_php_autoload::{AutoloadPath, ClassmapEntry, Psr4Mapping, Psr4Prefix};

let mapping = Psr4Mapping::new(Psr4Prefix::new("App\\")?)
    .with_path(AutoloadPath::new("src/")?);
let classmap = ClassmapEntry::new("Legacy_Class", AutoloadPath::new("legacy/Legacy_Class.php")?);

assert_eq!(mapping.prefix().as_str(), "App\\");
assert_eq!(classmap.class_name(), "Legacy_Class");
# Ok::<(), use_php_autoload::PhpAutoloadError>(())
```

## Scope

- PSR-4 prefix mappings, classmap entries, files entries, and autoload strategy labels.

## Non-goals

- Composer JSON parsing, package resolution, filesystem scanning, or runtime autoloading.

## License

Licensed under either Apache-2.0 or MIT.
