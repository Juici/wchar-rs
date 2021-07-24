//! This library introduces two macros [`wch`] and [`wchz`] to create UTF-16 or
//! UTF-32 wide strings at compiler time, like `L` string literals in C.
//!
//! # Example
//!
//! ```
//! use wchar::{wch, wchz, wchar_t};
//!
//! // Equivalent to `#define RUST L"Rust"` in C.
//! const RUST: &[wchar_t] = wch!("Rust\0"); // C strings are nul-terminated.
//! // Equivalent to `#define ALSO_RUST L"Rust"` in C.
//! const ALSO_RUST: &[wchar_t] = wchz!("Rust");
//!
//! assert_eq!(RUST, &['R' as wchar_t, 'u' as wchar_t, 's' as wchar_t, 't' as wchar_t, 0x0000]);
//! assert_eq!(RUST, ALSO_RUST);
//! ```

#![no_std]

pub use wchar_impl;

/// Platform wide character type.
#[allow(non_camel_case_types)]
#[cfg(target_os = "windows")]
pub type wchar_t = u16;
#[allow(non_camel_case_types)]
#[cfg(not(target_os = "windows"))]
pub type wchar_t = i32;

/// Generate a UTF-16 or UTF-32 wide string from a string literal.
///
/// The generated output takes the form of a slice of wide characters.
///
/// The first argument is the output character type, if no type is specified the
/// platform native `wchar_t` will be used.
///
/// # Notes
///
/// Whilst this macro can be used for C-style nul-terminated wide strings, no
/// validations are made about internal nul characters. If your strings need to
/// be nul-terminated it is recommended to use [`wchz`].
///
/// # Examples
///
/// Basic usage (platform native):
///
/// ```
/// # use wchar::{wch, wchar_t};
/// const WIDE: &[wchar_t] = wch!("foo");
/// ```
///
/// UTF-16 usage:
///
/// ```
/// # use wchar::wch;
/// let wide_str = wch!(u16, "foo");
/// let expected = &[0x0066, 0x006F, 0x006F];
///
/// assert_eq!(wide_str, expected);
/// ```
///
/// UTF-32 usage:
///
/// ```
/// # use wchar::wch;
/// let wide_str = wch!(u32, "foo");
/// let expected = &[0x0000_0066, 0x0000_006F, 0x0000_006F];
///
/// assert_eq!(wide_str, expected);
/// ```
#[macro_export]
#[cfg(target_os = "windows")]
macro_rules! wch {
    ($lit:literal) => {
		$crate::wch!(u16, $lit)
    };
    ($ty:ty, $lit:literal) => {
        $crate::wchar_impl::wch!($ty, $lit)
    };
}
#[macro_export]
#[cfg(not(target_os = "windows"))]
macro_rules! wch {
    ($lit:literal) => {
		$crate::wch!(i32, $lit)
    };
    ($ty:ty, $lit:literal) => {
        $crate::wchar_impl::wch!($ty, $lit)
    };
}

/// Generate a C-style nul-terminated UTF-16 or UTF-32 wide string from a
/// string literal.
///
/// Validations are made that the given string does not contain nul characters.
///
/// The generated output takes the form of a slice of wide characters, with a
/// nul-terminator as the last wide character.
///
/// The first argument is the output character type, if no type is specified the
/// platform native `wchar_t` will be used.
///
/// # Examples
///
/// Basic usage (platform native):
///
/// ```
/// # use wchar::{wchz, wchar_t};
/// const WIDE: &[wchar_t] = wchz!("foo");
/// ```
///
/// UTF-16 usage:
///
/// ```
/// # use wchar::wchz;
/// let wide_str = wchz!(u16, "bar");
/// let expected = &[0x0062, 0x0061, 0x0072, 0x0000];
///
/// assert_eq!(wide_str, expected);
/// ```
///
/// UTF-32 usage:
///
/// ```
/// # use wchar::wchz;
/// let wide_str = wchz!(u32, "bar");
/// let expected = &[0x0000_0062, 0x0000_0061, 0x0000_0072, 0x0000_0000];
///
/// assert_eq!(wide_str, expected);
/// ```
#[macro_export]
#[cfg(target_os = "windows")]
macro_rules! wchz {
    ($lit:literal) => {
		$crate::wchz!(u16, $lit)
    };
    ($ty:ty, $lit:literal) => {
        $crate::wchar_impl::wchz!($ty, $lit)
    };
}
#[macro_export]
#[cfg(not(target_os = "windows"))]
macro_rules! wchz {
    ($lit:literal) => {
		$crate::wchz!(i32, $lit)
    };
    ($ty:ty, $lit:literal) => {
        $crate::wchar_impl::wchz!($ty, $lit)
    };
}

/// Generate a UTF-16 or UTF-32 wide string from a UTF-8 encoded file.
///
/// The generated output takes the form of a slice of wide characters.
///
/// The first argument is the output character type, if no type is specified the
/// platform native `wchar_t` will be used.
///
/// # Notes
///
/// Whilst this macro can be used for C-style nul-terminated wide strings, no
/// validations are made about internal nul characters. If your strings need to
/// be nul-terminated it is recommended to use [`include_wchz`].
#[macro_export]
#[cfg(target_os = "windows")]
macro_rules! include_wch {
    ($lit:literal) => {
		$crate::include_wch!(u16, $lit)
    };
    ($ty:ty, $lit:literal) => {
        $crate::wchar_impl::include_wch!($ty, $lit)
    };
}
#[macro_export]
#[cfg(not(target_os = "windows"))]
macro_rules! include_wch {
    ($lit:literal) => {
		$crate::include_wch!(i32, $lit)
    };
    ($ty:ty, $lit:literal) => {
        $crate::wchar_impl::include_wch!($ty, $lit)
    };
}

/// Generate a UTF-16 or UTF-32 wide string from a UTF-8 encoded file.
///
/// Validations are made that the given string does not contain nul characters.
///
/// The generated output takes the form of a slice of wide characters, with a
/// nul-terminator as the last wide character.
///
/// The first argument is the output character type, if no type is specified the
/// platform native `wchar_t` will be used.
#[macro_export]
#[cfg(target_os = "windows")]
macro_rules! include_wchz {
    ($lit:literal) => {
		$crate::include_wch!(u16, $lit)
    };
    ($ty:ty, $lit:literal) => {
        $crate::wchar_impl::include_wchz!($ty, $lit)
    };
}
#[macro_export]
#[cfg(not(target_os = "windows"))]
macro_rules! include_wchz {
    ($lit:literal) => {
		$crate::include_wchz!(i32, $lit)
    };
    ($ty:ty, $lit:literal) => {
        $crate::wchar_impl::include_wchz!($ty, $lit)
    };
}
