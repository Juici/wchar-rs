//! A procedural macro for compile time UTF-16 wide strings.
//!
//! This crate introduces two macros [`wch!`] and [`wch_c!`] to create UTF-16
//! wide strings at compiler time, like the `L` literal in C.
//!
//! In order to use the macro the `proc_macro` and `proc_macro_non_items`
//! features must be enabled.
//!
//! [`wch!`]: fn.wch.html
//! [`wch_c!`]: fn.wch_c.html
//!
//! # Examples
//!
//! ```
//! extern crate wchar;
//!
//! use wchar::{wch, wch_c};
//!
//! // Equivalent to `#define RUST L"Rust"` in C.
//! const RUST: &[u16] = wch!("Rust\0"); // C strings are null-terminated.
//! assert_eq!(RUST, &[0x0052, 0x0075, 0x0073, 0x0074, 0x0000]);
//!
//! // Equivalent to `#define ALSO_RUST L"Rust"` in C.
//! const ALSO_RUST: &[u16] = wch_c!("Rust");
//! assert_eq!(ALSO_RUST, &[0x0052, 0x0075, 0x0073, 0x0074, 0x0000]);
//! ```

use proc_macro_hack::proc_macro_hack;

/// Generate a UTF-16 wide string from the given string literal.
///
/// The generated output takes the form of a slice of wide characters
/// `&'static [u16]`.
///
/// Whilst `wch!` can be used for C-style nul-terminated wide strings, no
/// validations are made about internal nul characters. For more complex use
/// cases it is recommended to use [`wch_c!`].
///
/// [`wch_c!`]: fn.wch_c.html
///
/// # Examples
///
/// Basic example:
///
/// ```
/// # use wchar::wch;
/// let wide_str = wch!("foo");
/// let expected = &[0x0066, 0x006F, 0x006F];
///
/// assert_eq!(wide_str, expected);
/// ```
///
/// Nul-terminated example:
///
/// ```
/// # use wchar::wch;
/// let wide_str = wch!("bar\0");
/// let expected = &[0x0062, 0x0061, 0x0072, 0x0000];
///
/// assert_eq!(wide_str, expected);
/// ```
#[proc_macro_hack]
pub use wchar_impl::wch;

/// Generate a C-style nul-terminated UTF-16 wide string from the given string
/// literal.
///
/// The generated output takes the form of a slice of wide characters
/// `&'static [u16]`, with a nul-terminator as the last wide character.
///
/// Validations are made that the given string does not contain nul characters.
///
/// # Examples
///
/// Basic example:
///
/// ```
/// # use wchar::wch_c;
/// let wide_str = wch_c!("bar");
/// let expected = &[0x0062, 0x0061, 0x0072, 0x0000];
///
/// assert_eq!(wide_str, expected);
/// ```
///
/// Raw literal example:
///
/// ```
/// # use wchar::wch_c;
/// let wide_str = wch_c!(r#"%HOME%\foo\bar"#);
/// let expected = &[
///     0x0025, 0x0048, 0x004F, 0x004D, 0x0045, 0x0025, 0x005C, 0x0066, 0x006F, 0x006F, 0x005C,
///     0x0062, 0x0061, 0x0072, 0x0000,
/// ];
///
/// assert_eq!(wide_str, expected);
/// ```
#[proc_macro_hack]
pub use wchar_impl::wch_c;
