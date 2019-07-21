extern crate proc_macro;

use std::iter::once;

use proc_macro_hack::proc_macro_hack;
use syn::LitStr;

#[proc_macro_hack]
pub fn wch(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let lit: LitStr = syn::parse_macro_input!(input as LitStr);

    let string = lit.value();
    let bytes: Vec<u16> = encode_wide(&string);

    let expanded = quote::quote! {
        &[#(#bytes),*]
    };

    expanded.into()
}

#[proc_macro_hack]
pub fn wch_c(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let lit: LitStr = syn::parse_macro_input!(input as LitStr);

    let string = lit.value();

    if string.contains('\0') {
        return syn::Error::new(lit.span(), "C-style string cannot contain nul characters")
            .to_compile_error()
            .into();
    }

    let bytes: Vec<u16> = encode_wide_c(&string);

    let expanded = quote::quote! {
        &[#(#bytes),*]
    };

    expanded.into()
}

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

#[cfg(windows)]
fn encode_wide_c(s: &str) -> Vec<u16> {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    OsStr::new(s).encode_wide().chain(once(0)).collect()
}

#[cfg(not(windows))]
fn encode_wide_c(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(once(0)).collect()
}
