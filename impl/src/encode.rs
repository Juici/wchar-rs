use std::any::type_name;
use std::iter::once;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Error, LitChar, Result};

use crate::parse::WCharType;

pub fn expand_char(ty: Option<WCharType>, c: LitChar) -> Result<TokenStream> {
    fn quote_char<T: Encode>(c: LitChar) -> Result<TokenStream> {
        match T::encode_char(c.value()) {
            Some(c) => Ok(quote::quote! { #c }),
            None => Err(Error::new(
                c.span(),
                format_args!(
                    "character does not fit within a {} wide character",
                    type_name::<T>()
                ),
            )),
        }
    }

    match ty {
        Some(WCharType::U16(_)) => quote_char::<u16>(c),
        Some(WCharType::U32(_)) => quote_char::<u32>(c),
        Some(WCharType::I16(_)) => quote_char::<i16>(c),
        Some(WCharType::I32(_)) => quote_char::<i32>(c),
        None => quote_char::<libc::wchar_t>(c),
    }
}

pub fn expand_str(ty: Option<WCharType>, text: &str) -> TokenStream {
    fn quote_str<T: Encode>(text: &str) -> TokenStream {
        let chars = T::encode_str(text);
        quote::quote! { &[#(#chars),*] }
    }

    match ty {
        Some(WCharType::U16(_)) => quote_str::<u16>(text),
        Some(WCharType::U32(_)) => quote_str::<u32>(text),
        Some(WCharType::I16(_)) => quote_str::<i16>(text),
        Some(WCharType::I32(_)) => quote_str::<i32>(text),
        None => quote_str::<libc::wchar_t>(text),
    }
}

pub fn expand_str_c(ty: Option<WCharType>, text: &str) -> TokenStream {
    fn quote_str_c<T: Encode>(text: &str) -> TokenStream {
        let chars = T::encode_str_c(text);
        quote::quote! { &[#(#chars),*] }
    }

    match ty {
        Some(WCharType::U16(_)) => quote_str_c::<u16>(text),
        Some(WCharType::U32(_)) => quote_str_c::<u32>(text),
        Some(WCharType::I16(_)) => quote_str_c::<i16>(text),
        Some(WCharType::I32(_)) => quote_str_c::<i32>(text),
        None => quote_str_c::<libc::wchar_t>(text),
    }
}

pub trait Encode: Copy + ToTokens {
    fn wchar_type() -> WCharType;

    fn encode_char(c: char) -> Option<Self>;

    fn encode_str(s: &str) -> Vec<Self>;

    fn encode_str_c(s: &str) -> Vec<Self>;
}

impl Encode for u16 {
    fn wchar_type() -> WCharType {
        syn::parse_quote!(u16)
    }

    fn encode_char(c: char) -> Option<Self> {
        if c.len_utf16() == 1 {
            let mut buf = [0; 1];
            c.encode_utf16(&mut buf);
            Some(buf[0])
        } else {
            None
        }
    }

    fn encode_str(s: &str) -> Vec<Self> {
        s.encode_utf16().collect()
    }

    fn encode_str_c(s: &str) -> Vec<Self> {
        s.encode_utf16().chain(once(0)).collect()
    }
}

impl Encode for u32 {
    fn wchar_type() -> WCharType {
        syn::parse_quote!(u32)
    }

    fn encode_char(c: char) -> Option<Self> {
        Some(c as u32)
    }

    fn encode_str(s: &str) -> Vec<Self> {
        s.chars().map(|c| c as u32).collect()
    }

    fn encode_str_c(s: &str) -> Vec<Self> {
        s.chars().map(|c| c as u32).chain(once(0)).collect()
    }
}

impl Encode for i16 {
    fn wchar_type() -> WCharType {
        syn::parse_quote!(i16)
    }

    fn encode_char(c: char) -> Option<Self> {
        u16::encode_char(c).map(|c| c as i16)
    }

    fn encode_str(s: &str) -> Vec<Self> {
        s.encode_utf16().map(|c| c as i16).collect()
    }

    fn encode_str_c(s: &str) -> Vec<Self> {
        s.encode_utf16().map(|c| c as i16).chain(once(0)).collect()
    }
}

impl Encode for i32 {
    fn wchar_type() -> WCharType {
        syn::parse_quote!(i32)
    }

    fn encode_char(c: char) -> Option<Self> {
        Some(c as i32)
    }

    fn encode_str(s: &str) -> Vec<Self> {
        s.chars().map(|c| c as i32).collect()
    }

    fn encode_str_c(s: &str) -> Vec<Self> {
        s.chars().map(|c| c as i32).chain(once(0)).collect()
    }
}
