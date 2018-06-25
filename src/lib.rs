//! A procedural macro for compile time UTF-16 strings.
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
//! #![feature(proc_macro, proc_macro_non_items)]
//!
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

#![feature(proc_macro)]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;

use std::fmt::Write;
use std::iter::once;

use proc_macro2::TokenStream;
use syn::LitStr;

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
/// # #![feature(proc_macro, proc_macro_non_items)]
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
/// # #![feature(proc_macro, proc_macro_non_items)]
/// # use wchar::wch;
/// let wide_str = wch!("bar\0");
/// let expected = &[0x0062, 0x0061, 0x0072, 0x0000];
///
/// assert_eq!(wide_str, expected);
/// ```
#[proc_macro]
pub fn wch(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();

    let lit = match syn::parse2::<LitStr>(input) {
        Ok(lit) => lit,
        Err(_) => panic!("expected a string literal"),
    };
    let data: String = lit.value();

    let wide_string = WideString::from_str(&data);
    wide_string.generate_code().parse().unwrap()
}

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
/// # #![feature(proc_macro, proc_macro_non_items)]
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
/// # #![feature(proc_macro, proc_macro_non_items)]
/// # use wchar::wch_c;
/// let wide_str = wch_c!(r#"%HOME%\foo\bar"#);
/// let expected = &[
///     0x0025, 0x0048, 0x004F, 0x004D, 0x0045, 0x0025, 0x005C, 0x0066, 0x006F, 0x006F, 0x005C,
///     0x0062, 0x0061, 0x0072, 0x0000,
/// ];
///
/// assert_eq!(wide_str, expected);
/// ```
#[proc_macro]
pub fn wch_c(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();

    let lit = match syn::parse2::<LitStr>(input) {
        Ok(lit) => lit,
        Err(_) => panic!("expected a string literal"),
    };
    let data: String = lit.value();

    let wide_string = WideCString::from_str(&data);
    wide_string.generate_code().parse().unwrap()
}

struct WideString {
    inner: Vec<u16>,
}

impl WideString {
    fn from_str(s: &str) -> WideString {
        #[cfg(windows)]
        fn encode_wide(s: &str) -> Vec<u16> {
            use std::ffi::OsStr;
            use std::os::windows::ffi::OsStrExt;

            OsStr::new(s).encode_wide().collect()
        }

        #[cfg(not(windows))]
        fn encode_wide(s: &str) -> Vec<u16> {
            s.encode_utf16().collect()
        }

        WideString {
            inner: encode_wide(s),
        }
    }

    fn generate_code(&self) -> String {
        generate_slice_code(&self.inner)
    }
}

struct WideCString {
    inner: Vec<u16>,
}

impl WideCString {
    fn from_str(s: &str) -> WideCString {
        if s.contains('\0') {
            panic!("string cannot contain nul characters");
        }

        #[cfg(windows)]
        fn encode_wide(s: &str) -> Vec<u16> {
            use std::ffi::OsStr;
            use std::os::windows::ffi::OsStrExt;

            OsStr::new(s).encode_wide().chain(once(0)).collect()
        }

        #[cfg(not(windows))]
        fn encode_wide(s: &str) -> Vec<u16> {
            s.encode_utf16().chain(once(0)).collect()
        }

        WideCString {
            inner: encode_wide(s),
        }
    }

    fn generate_code(&self) -> String {
        generate_slice_code(&self.inner)
    }
}

fn generate_slice_code(slice: &[u16]) -> String {
    let len = slice.len();
    // Allocate exact space for final result.
    let mut s = String::with_capacity(1 + 8 * len);

    s.push_str("&[");

    let mut i = 0;
    while i < len {
        if i > 0 {
            s.push_str(", ");
        }
        write!(s, "{:#06X}", &slice[i]).unwrap();
        i += 1;
    }

    s.push(']');

    s
}
