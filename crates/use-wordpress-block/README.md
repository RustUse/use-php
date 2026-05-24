# use-wordpress-block

WordPress block metadata primitives for `RustUse`.

## Experimental

`use-wordpress-block` is experimental while `use-php` remains below `0.3.0`.

## Example

```rust
use use_wordpress_block::{WordPressBlockJson, WordPressBlockName};

let block = WordPressBlockJson::new(WordPressBlockName::new("acme/book-card")?);

assert_eq!(block.name().as_str(), "acme/book-card");
# Ok::<(), use_wordpress_block::WordPressBlockError>(())
```

## Scope

- `block.json`-oriented block names, categories, attributes, supports, and asset path labels.

## Non-goals

- JavaScript, Gutenberg runtime behavior, block rendering, or asset building.

## License

Licensed under either Apache-2.0 or MIT.
