extern crate proc_macro;

use std::iter::once;

use quote::ToTokens;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Error, Ident, LitStr, Token};

enum WCharType {
    U16,
    U32,
    I16,
    I32,
}

struct Input {
    ty: WCharType,
    lit: LitStr,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        let ty = if lookahead.peek(Ident) {
            let ty = input.parse::<Ident>()?;
            let _ = input.parse::<Token![,]>()?;

            let s = ty.to_string();
            match &s[..] {
                "u16" => WCharType::U16,
                "u32" => WCharType::U32,
                "i16" => WCharType::I16,
                "i32" => WCharType::I32,
                _ => {
                    return Err(Error::new(
                        ty.span(),
                        format_args!(
                            "unexpected wchar type `{}`, expected one of: u16, u32, i16, i32",
                            s
                        ),
                    ));
                }
            }
        } else if lookahead.peek(LitStr) {
            // Default to u16 as the wide character.
            WCharType::U16
        } else {
            // We expect the wide character type or the string.
            return Err(lookahead.error());
        };

        let lit = input.parse()?;

        Ok(Input { ty, lit })
    }
}

#[proc_macro]
pub fn wch(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Input { ty, lit } = syn::parse_macro_input!(input);

    let string = lit.value();

    // Warn the user that they should probably be using wch_c.
    // if let Some(0) = string.as_bytes().last() {
    //     lit.span().unwrap().warning("string is terminated by nul character, perhaps you should use wch_c");
    // }

    let expanded = match ty {
        WCharType::U16 => {
            let chars = u16::encode_wide(&string);
            quote::quote! { &[#(#chars),*] }
        }
        WCharType::U32 => {
            let chars = u32::encode_wide(&string);
            quote::quote! { &[#(#chars),*] }
        }
        WCharType::I16 => {
            let chars = i16::encode_wide(&string);
            quote::quote! { &[#(#chars),*] }
        }
        WCharType::I32 => {
            let chars = i32::encode_wide(&string);
            quote::quote! { &[#(#chars),*] }
        }
    };

    expanded.into()
}

#[proc_macro]
pub fn wch_c(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Input { ty, lit } = syn::parse_macro_input!(input);

    let string = lit.value();

    if string.contains('\0') {
        return syn::Error::new(lit.span(), "C-style string cannot contain nul characters")
            .to_compile_error()
            .into();
    }

    let expanded = match ty {
        WCharType::U16 => {
            let chars = u16::encode_wide_c(&string);
            quote::quote! { &[#(#chars),*] }
        }
        WCharType::U32 => {
            let chars = u32::encode_wide_c(&string);
            quote::quote! { &[#(#chars),*] }
        }
        WCharType::I16 => {
            let chars = i16::encode_wide_c(&string);
            quote::quote! { &[#(#chars),*] }
        }
        WCharType::I32 => {
            let chars = i32::encode_wide_c(&string);
            quote::quote! { &[#(#chars),*] }
        }
    };

    expanded.into()
}

trait Encode: Sized + ToTokens {
    fn encode_wide(s: &str) -> Vec<Self>;
    fn encode_wide_c(s: &str) -> Vec<Self>;
}

impl Encode for u16 {
    fn encode_wide(s: &str) -> Vec<Self> {
        s.encode_utf16().collect()
    }

    fn encode_wide_c(s: &str) -> Vec<Self> {
        s.encode_utf16().chain(once(0)).collect()
    }
}

impl Encode for u32 {
    fn encode_wide(s: &str) -> Vec<Self> {
        s.chars().map(|c| c as u32).collect()
    }

    fn encode_wide_c(s: &str) -> Vec<Self> {
        s.chars().map(|c| c as u32).chain(once(0)).collect()
    }
}

impl Encode for i16 {
    fn encode_wide(s: &str) -> Vec<Self> {
        s.encode_utf16().map(|c| c as i16).collect()
    }

    fn encode_wide_c(s: &str) -> Vec<Self> {
        s.encode_utf16().map(|c| c as i16).chain(once(0)).collect()
    }
}

impl Encode for i32 {
    fn encode_wide(s: &str) -> Vec<Self> {
        s.chars().map(|c| c as i32).collect()
    }

    fn encode_wide_c(s: &str) -> Vec<Self> {
        s.chars().map(|c| c as i32).chain(once(0)).collect()
    }
}
