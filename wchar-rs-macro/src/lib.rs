extern crate proc_macro;

use proc_macro2;
use proc_macro2::TokenStream;
use proc_macro_hack::proc_macro_hack;
use quote::quote;
use std::iter::once;
use syn;
use syn::LitStr;

#[proc_macro_hack]
pub fn wch(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();

    let lit = match syn::parse2::<LitStr>(input) {
        Ok(lit) => lit,
        Err(_) => panic!("expected a string literal"),
    };
    let data: String = lit.value();

    let wide_string = WideString::from_str(&data);
    wide_string.generate_code()
}

#[proc_macro_hack]
pub fn wch_c(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();

    let lit = match syn::parse2::<LitStr>(input) {
        Ok(lit) => lit,
        Err(_) => panic!("expected a string literal"),
    };
    let data: String = lit.value();

    let wide_string = WideCString::from_str(&data);
    wide_string.generate_code()
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

    fn generate_code(&self) -> proc_macro::TokenStream {
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

    fn generate_code(&self) -> proc_macro::TokenStream {
        generate_slice_code(&self.inner)
    }
}

fn generate_slice_code(slice: &[u16]) -> proc_macro::TokenStream {
    (quote! {
        &[#(#slice),*]
    })
    .into()
}
