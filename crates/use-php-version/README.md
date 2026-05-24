# use-php-version

PHP version metadata primitives for `RustUse`.

## Experimental

`use-php-version` is experimental while `use-php` remains below `0.3.0`.

## Example

```rust
use use_php_version::{PhpSupportPhase, PhpVersion, PhpVersionBranch};

let version: PhpVersion = "8.3.2".parse()?;
let branch = PhpVersionBranch::from_version(&version)?;

assert_eq!(version.major(), 8);
assert_eq!(branch.to_string(), "8.3");
assert_eq!(PhpSupportPhase::Security.as_str(), "security");
# Ok::<(), use_php_version::PhpVersionParseError>(())
```

## Scope

- PHP version components, full version labels, and branch labels.
- Lightweight comparison through derived ordering.
- Static support phase labels such as active, security, end-of-life, and unknown.

## Non-goals

- Release calendar tracking or volatile support-date data.
- Composer constraint solving.

## License

Licensed under either Apache-2.0 or MIT.
