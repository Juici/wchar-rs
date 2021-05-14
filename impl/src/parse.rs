use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Lookahead1, Parse, ParseStream, Result};
use syn::{LitChar, LitStr, Token};

mod kw {
    syn::custom_keyword!(u16);
    syn::custom_keyword!(u32);
    syn::custom_keyword!(i16);
    syn::custom_keyword!(i32);
}

pub enum WCharType {
    U16(kw::u16),
    U32(kw::u32),
    I16(kw::i16),
    I32(kw::i32),
}

impl WCharType {
    fn peek(lookahead: &Lookahead1) -> bool {
        lookahead.peek(kw::u16)
            || lookahead.peek(kw::u32)
            || lookahead.peek(kw::i16)
            || lookahead.peek(kw::i32)
    }
}

impl Parse for WCharType {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::u16) {
            Ok(WCharType::U16(input.parse()?))
        } else if lookahead.peek(kw::u32) {
            Ok(WCharType::U32(input.parse()?))
        } else if lookahead.peek(kw::i16) {
            Ok(WCharType::I16(input.parse()?))
        } else if lookahead.peek(kw::i32) {
            Ok(WCharType::I32(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl ToTokens for WCharType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            WCharType::U16(ty) => ty.to_tokens(tokens),
            WCharType::U32(ty) => ty.to_tokens(tokens),
            WCharType::I16(ty) => ty.to_tokens(tokens),
            WCharType::I32(ty) => ty.to_tokens(tokens),
        }
    }
}

pub enum LitStrOrChar {
    Str(LitStr),
    Char(LitChar),
}

impl Parse for LitStrOrChar {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(LitStr) {
            Ok(LitStrOrChar::Str(input.parse()?))
        } else if lookahead.peek(LitChar) {
            Ok(LitStrOrChar::Char(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

pub struct WchInput {
    pub ty: Option<(WCharType, Token![,])>,
    pub literal: LitStrOrChar,
}

impl Parse for WchInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        let ty = if WCharType::peek(&lookahead) {
            Some((input.parse()?, input.parse()?))
        } else if lookahead.peek(LitStr) || lookahead.peek(LitChar) {
            None
        } else {
            return Err(lookahead.error());
        };

        Ok(WchInput {
            ty,
            literal: input.parse()?,
        })
    }
}

pub struct WchzInput {
    pub ty: Option<(WCharType, Token![,])>,
    pub literal: LitStr,
}

impl Parse for WchzInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        let ty = if WCharType::peek(&lookahead) {
            Some((input.parse()?, input.parse()?))
        } else if lookahead.peek(LitStr) {
            None
        } else {
            return Err(lookahead.error());
        };

        Ok(WchzInput {
            ty,
            literal: input.parse()?,
        })
    }
}

pub struct IncludeInput {
    pub ty: Option<(WCharType, Token![,])>,
    pub file_path: LitStr,
}

impl Parse for IncludeInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        let ty = if WCharType::peek(&lookahead) {
            Some((input.parse()?, input.parse()?))
        } else if lookahead.peek(LitStr) {
            None
        } else {
            return Err(lookahead.error());
        };

        Ok(IncludeInput {
            ty,
            file_path: input.parse()?,
        })
    }
}
