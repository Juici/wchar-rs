# wchar

[<img alt="build status" src="https://img.shields.io/github/workflow/status/Juici/wchar-rs/ci?style=for-the-badge" height="20">](https://github.com/Juici/wchar-rs/actions?query=branch%3Amaster)
[<img alt="crates.io" src="https://img.shields.io/crates/v/wchar?style=for-the-badge&logo=rust" height="20">](https://crates.io/crates/wchar)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-wchar-4d76ae?style=for-the-badge&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCA1MTIgNTEyIj48cGF0aCBmaWxsPSIjZmZmIiBkPSJNNDg4LjYgMjUwLjJMMzkyIDIxNFYxMDUuNWMwLTE1LTkuMy0yOC40LTIzLjQtMzMuN2wtMTAwLTM3LjVjLTguMS0zLjEtMTcuMS0zLjEtMjUuMyAwbC0xMDAgMzcuNWMtMTQuMSA1LjMtMjMuNCAxOC43LTIzLjQgMzMuN1YyMTRsLTk2LjYgMzYuMkM5LjMgMjU1LjUgMCAyNjguOSAwIDI4My45VjM5NGMwIDEzLjYgNy43IDI2LjEgMTkuOSAzMi4ybDEwMCA1MGMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAzLjktNTIgMTAzLjkgNTJjMTAuMSA1LjEgMjIuMSA1LjEgMzIuMiAwbDEwMC01MGMxMi4yLTYuMSAxOS45LTE4LjYgMTkuOS0zMi4yVjI4My45YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43ek0zNTggMjE0LjhsLTg1IDMxLjl2LTY4LjJsODUtMzd2NzMuM3pNMTU0IDEwNC4xbDEwMi0zOC4yIDEwMiAzOC4ydi42bC0xMDIgNDEuNC0xMDItNDEuNHYtLjZ6bTg0IDI5MS4xbC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnptMjQwIDExMmwtODUgNDIuNXYtNzkuMWw4NS0zOC44djc1LjR6bTAtMTEybC0xMDIgNDEuNC0xMDItNDEuNHYtLjZsMTAyLTM4LjIgMTAyIDM4LjJ2LjZ6Ij48L3BhdGg+PC9zdmc+" height="20">](https://docs.rs/wchar)

This library introduces two macros to create UTF-16 and UTF-32 wide strings at
compiler time, like `L` string literals in C.

```toml
[dependencies]
wchar = "0.10"
```

*Compiler support: requires rustc 1.53+*

## Example

```rust
use wchar::{wch, wchz, wchar_t};

// Equivalent to `#define RUST L"Rust"` in C.
const RUST: &[wchar_t] = wch!("Rust\0"); // C strings are nul-terminated.
// Equivalent to `#define ALSO_RUST L"Rust"` in C.
const ALSO_RUST: &[wchar_t] = wchz!("Rust");

assert_eq!(RUST, &['R' as wchar_t, 'u' as wchar_t, 's' as wchar_t, 't' as wchar_t, 0x0000]);
assert_eq!(RUST, ALSO_RUST);
```

## License

This project is licensed under either of [Apache License, Version 2.0](LICENSE-APACHE)
or [MIT License](LICENSE-MIT) at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
