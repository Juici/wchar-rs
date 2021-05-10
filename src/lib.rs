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

#[doc(hidden)]
pub use core as _core;
#[doc(hidden)]
pub use wchar_impl as _impl;

cfg_if::cfg_if! {
    if #[cfg(wchar_t = "u16")] {
        /// Platform wide character type.
        #[allow(non_camel_case_types)]
        pub type wchar_t = u16;

        #[doc(hidden)]
        #[macro_export]
        macro_rules! __expand_platform_wchar {
            ($macro:ident, $string:literal) => {
                $crate::_impl::$macro!(u16, $string)
            };
        }
    } else if #[cfg(wchar_t = "u32")] {
        /// Platform wide character type.
        #[allow(non_camel_case_types)]
        pub type wchar_t = u32;

        #[doc(hidden)]
        #[macro_export]
        macro_rules! __expand_platform_wchar {
            ($macro:ident, $string:literal) => {
                $crate::_impl::$macro!(u32, $string)
            };
        }
    } else if #[cfg(wchar_t = "i32")] {
        /// Platform wide character type.
        #[allow(non_camel_case_types)]
        pub type wchar_t = i32;

        #[doc(hidden)]
        #[macro_export]
        macro_rules! __expand_platform_wchar {
            ($macro:ident, $string:literal) => {
                $crate::_impl::$macro!(i32, $string)
            };
        }
    } else {
        #[doc(hidden)]
        #[macro_export]
        macro_rules! __expand_platform_wchar {
            ($macro:ident, $string:literal) => {
                $crate::_core::compile_error!("native wchar_t not support for this platform");
            };
        }
    }
}

/// Generate a UTF-16 or UTF-32 wide string from a string literal.
///
/// The generated output takes the form of a slice of wide characters.
///
/// The first argument is the output character type, if no type is specfied the
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
macro_rules! wch {
    ($ty:ident, $string:literal) => {
        $crate::_impl::wch!($ty, $string)
    };
    ($string:literal) => {
        $crate::__expand_platform_wchar!(wch, $string)
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
/// The first argument is the output character type, if no type is specfied the
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
macro_rules! wchz {
    ($ty:ident, $string:literal) => {
        $crate::_impl::wchz!($ty, $string)
    };
    ($string:literal) => {
        $crate::__expand_platform_wchar!(wchz, $string)
    };
}

/// Generate a UTF-16 or UTF-32 wide string from a UTF-8 encoded file.
///
/// The generated output takes the form of a slice of wide characters.
///
/// The first argument is the output character type, if no type is specfied the
/// platform native `wchar_t` will be used.
///
/// # Notes
///
/// Whilst this macro can be used for C-style nul-terminated wide strings, no
/// validations are made about internal nul characters. If your strings need to
/// be nul-terminated it is recommended to use [`include_wchz`].
///
/// # Examples
///
/// Basic usage (platform native):
///
/// ```
/// # use wchar::{include_wch, wchar_t};
/// const WIDE: &[wchar_t] = include_wch!("../README.md");
/// ```
#[cfg(feature = "nightly")]
#[macro_export]
macro_rules! include_wch {
    ($ty:ident, $string:literal) => {
        $crate::_impl::include_wch!($ty, $string)
    };
    ($string:literal) => {
        $crate::__expand_platform_wchar!(include_wch, $string)
    };
}

/// Generate a UTF-16 or UTF-32 wide string from a UTF-8 encoded file.
///
/// Validations are made that the given string does not contain nul characters.
///
/// The generated output takes the form of a slice of wide characters, with a
/// nul-terminator as the last wide character.
///
/// The first argument is the output character type, if no type is specfied the
/// platform native `wchar_t` will be used.
///
/// # Examples
///
/// Basic usage (platform native):
///
/// ```
/// # use wchar::{include_wchz, wchar_t};
/// const WIDE: &[wchar_t] = include_wchz!("../README.md");
/// ```
#[cfg(feature = "nightly")]
#[macro_export]
macro_rules! include_wchz {
    ($ty:ident, $string:literal) => {
        $crate::_impl::include_wch!($ty, $string)
    };
    ($string:literal) => {
        $crate::__expand_platform_wchar!(include_wchz, $string)
    };
}
