# use-php-docblock

PHPDoc block metadata primitives for `RustUse`.

## Experimental

`use-php-docblock` is experimental while `use-php` remains below `0.3.0`.

## Example

```rust
use use_php_docblock::{Docblock, DocblockTag, DocblockTagKind, TagName};

let block = Docblock::new("/**\n * Create a user.\n *\n * Stores metadata only.\n */")?;
let tag = DocblockTag::new(TagName::new("return")?).with_kind(DocblockTagKind::Return);

assert_eq!(block.summary(), "Create a user.");
assert_eq!(tag.kind(), Some(DocblockTagKind::Return));
# Ok::<(), use_php_docblock::PhpDocblockError>(())
```

## Scope

- PHPDoc summary/body metadata, common tag labels, tag names, and type-string wrappers.

## Non-goals

- Complete PHPDoc parsing, type-expression parsing, inheritance, or analyzer-specific dialects.

## License

Licensed under either Apache-2.0 or MIT.
