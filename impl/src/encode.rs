use std::any::type_name;
use std::iter::once;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{Error, LitChar, Result};

use crate::parse::WCharType;

pub fn expand_char(ty: WCharType, c: LitChar) -> Result<TokenStream> {
    fn quote_char<T: Wide>(c: LitChar) -> Result<TokenStream> {
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
        WCharType::U16(_) => quote_char::<u16>(c),
        WCharType::U32(_) => quote_char::<u32>(c),
        WCharType::I16(_) => quote_char::<i16>(c),
        WCharType::I32(_) => quote_char::<i32>(c),
    }
}

pub fn expand_str(ty: WCharType, text: &str) -> TokenStream {
    fn quote_str<T: Wide>(text: &str) -> TokenStream {
        let chars = T::encode_str(text);
        quote::quote! { &[#(#chars),*] }
    }

    match ty {
        WCharType::U16(_) => quote_str::<u16>(text),
        WCharType::U32(_) => quote_str::<u32>(text),
        WCharType::I16(_) => quote_str::<i16>(text),
        WCharType::I32(_) => quote_str::<i32>(text),
    }
}

pub fn expand_str_c(ty: WCharType, text: &str) -> TokenStream {
    fn quote_str_c<T: Wide>(text: &str) -> TokenStream {
        let chars = T::encode_str_c(text);
        quote::quote! { &[#(#chars),*] }
    }

    match ty {
        WCharType::U16(_) => quote_str_c::<u16>(text),
        WCharType::U32(_) => quote_str_c::<u32>(text),
        WCharType::I16(_) => quote_str_c::<i16>(text),
        WCharType::I32(_) => quote_str_c::<i32>(text),
    }
}

trait Wide: Copy + ToTokens {
    fn encode_char(c: char) -> Option<Self>;

    fn encode_str(s: &str) -> Vec<Self>;

    fn encode_str_c(s: &str) -> Vec<Self>;
}

impl Wide for u16 {
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

impl Wide for u32 {
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

impl Wide for i16 {
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

impl Wide for i32 {
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
