use std::iter::once;

use proc_macro2::TokenStream;
use quote::ToTokens;

use crate::parse::WCharType;

pub fn expand(ty: WCharType, text: &str) -> TokenStream {
    match ty {
        WCharType::U16 => {
            let chars = u16::encode_wide(&text);
            quote::quote! { &[#(#chars),*] }
        }
        WCharType::U32 => {
            let chars = u32::encode_wide(&text);
            quote::quote! { &[#(#chars),*] }
        }
        WCharType::I16 => {
            let chars = i16::encode_wide(&text);
            quote::quote! { &[#(#chars),*] }
        }
        WCharType::I32 => {
            let chars = i32::encode_wide(&text);
            quote::quote! { &[#(#chars),*] }
        }
    }
}

pub fn expand_c(ty: WCharType, text: &str) -> TokenStream {
    match ty {
        WCharType::U16 => {
            let chars = u16::encode_wide_c(&text);
            quote::quote! { &[#(#chars),*] }
        }
        WCharType::U32 => {
            let chars = u32::encode_wide_c(&text);
            quote::quote! { &[#(#chars),*] }
        }
        WCharType::I16 => {
            let chars = i16::encode_wide_c(&text);
            quote::quote! { &[#(#chars),*] }
        }
        WCharType::I32 => {
            let chars = i32::encode_wide_c(&text);
            quote::quote! { &[#(#chars),*] }
        }
    }
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
