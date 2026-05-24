# use-wordpress

WordPress metadata primitives for `RustUse`.

## Experimental

`use-wordpress` is experimental while `use-php` remains below `0.3.0`.

## Example

```rust
use use_wordpress::{WordPressPluginHeader, WordPressPluginHeaderName, WordPressText};

let header = WordPressPluginHeader::new(
    WordPressPluginHeaderName::new("Plugin Name")?,
    WordPressText::new("Example Tools")?,
);

assert_eq!(header.name().as_str(), "Plugin Name");
# Ok::<(), use_wordpress::WordPressError>(())
```

## Scope

- Plugin header fields, theme metadata fields, post type slugs, taxonomy slugs, capability strings, and REST namespace/route labels.

## Non-goals

- WordPress SDK behavior, REST clients, plugins, themes, hooks, blocks, or runtime integration.

## License

Licensed under either Apache-2.0 or MIT.
