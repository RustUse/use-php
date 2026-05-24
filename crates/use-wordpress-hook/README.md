# use-wordpress-hook

WordPress hook metadata primitives for `RustUse`.

## Experimental

`use-wordpress-hook` is experimental while `use-php` remains below `0.3.0`.

## Example

```rust
use use_wordpress_hook::{HookCallbackMetadata, HookKind, HookName, HookReference};

let hook = HookReference::new(HookName::new("init")?, HookKind::Action)
    .with_callback(HookCallbackMetadata::new("register_books"));

assert_eq!(hook.name().as_str(), "init");
assert_eq!(hook.kind(), HookKind::Action);
# Ok::<(), use_wordpress_hook::WordPressHookError>(())
```

## Scope

- Action/filter hook names, priorities, callback metadata, and hook references.

## Non-goals

- Runtime callback execution, plugin loading, or WordPress event dispatching.

## License

Licensed under either Apache-2.0 or MIT.
