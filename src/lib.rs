//! This library introduces two macros [`wch`] and [`wch_c`] to create UTF-16 or
//! UTF-32 wide strings at compiler time, like `L` string literals in C.
//!
//! # Example
//!
//! ```
//! use wchar::{wch, wch_c};
//!
//! // Equivalent to `#define RUST L"Rust"` in C.
//! const RUST: &[u16] = wch!("Rust\0"); // C strings are nul-terminated.
//! assert_eq!(RUST, &[0x0052, 0x0075, 0x0073, 0x0074, 0x0000]);
//!
//! // Equivalent to `#define ALSO_RUST L"Rust"` in C.
//! const ALSO_RUST: &[u16] = wch_c!("Rust");
//! assert_eq!(ALSO_RUST, &[0x0052, 0x0075, 0x0073, 0x0074, 0x0000]);
//! ```

#![no_std]

#[doc(hidden)]
pub use wchar_impl;

/// Generate a UTF-16 or UTF-32 wide string from the given string literal.
///
/// The generated output takes the form of a slice of wide characters.
///
/// # Notes
///
/// Whilst [`wch`] can be used for C-style nul-terminated wide strings, no
/// validations are made about internal nul characters. If your strings need to
/// be nul-terminated it is recommended to use [`wch_c`].
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
///
/// UTF-32 example:
///
/// ```
/// # use wchar::wch;
/// let wide_str = wch!(u32, "wide");
/// let expected = &['w' as u32, 'i' as u32, 'd' as u32, 'e' as u32];
///
/// assert_eq!(wide_str, expected);
/// ```
#[macro_export]
macro_rules! wch {
    ($string:literal) => {
        $crate::wch!(u16, $string)
    };
    ($ty:ident, $string:literal) => {
        $crate::wchar_impl::wch!($ty, $string)
    };
}

/// Generate a C-style nul-terminated UTF-16 or UTF-32 wide string from the
/// given string literal.
///
/// Validations are made that the given string does not contain nul characters.
///
/// The generated output takes the form of a slice of wide characters, with a
/// nul-terminator as the last wide character.
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
#[macro_export]
macro_rules! wch_c {
    ($string:literal) => {
        $crate::wch_c!(u16, $string)
    };
    ($ty:ident, $string:literal) => {
        $crate::wchar_impl::wch_c!($ty, $string)
    };
}
