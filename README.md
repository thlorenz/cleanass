## cleanass <img src="assets/cleanass.png" width="150" height="150" align="right" />

[![Crates.io](https://img.shields.io/crates/v/cleanass)](https://crates.io/crates/cleanass)
[![Docs.rs](https://docs.rs/cleanass/badge.svg)](https://docs.rs/cleanass)

Enhances assert, assert_eq and assert_ne with cleanup closure which runs on failure.

### Usage

Import `use cleanass::{assert_eq, assert_ne, assert};` and use them as you would the built-in
versions or pass in a closure which runs whenever an assertion fails.


```rust
use cleanass::assert_ne;

pub fn main() {
    // If assert succeeds nothing is printed since cleanup function does not run
    {
        let a = 1;
        let b = 2;
        assert_ne!(a, b, || eprintln!("Cleanup: {} != {} succeeded", a, b));
    }
    // If assert fails the cleanup function runs and prints the message
    {
        let a = 1;
        let b = 1;
        assert_ne!(
            a, b,
            || eprintln!("Cleanup: {} != {} failed", a, b),
            "Should not be equal"
        );
    }
}
```

## Features

### `strict`

- disables versions of the assertions that don't require a closure ensuring that you don't
accidentally forget to add a cleanup function
- is on by default

## LICENSE

MIT
