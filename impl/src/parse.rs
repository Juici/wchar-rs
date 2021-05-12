use proc_macro2::Ident;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Error, LitChar, LitStr, Token};

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
            // HACK: Provide nicer compiler errors.
            let mut err = lookahead.error();
            if let Ok(ident) = input.parse::<Ident>() {
                err = Error::new(ident.span(), err);
            }
            Err(err)
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
    pub ty: WCharType,
    pub comma: Token![,],
    pub literal: LitStrOrChar,
}

impl Parse for WchInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(WchInput {
            ty: input.parse()?,
            comma: input.parse()?,
            literal: input.parse()?,
        })
    }
}

pub struct WchzInput {
    pub ty: WCharType,
    pub comma: Token![,],
    pub literal: LitStr,
}

impl Parse for WchzInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(WchzInput {
            ty: input.parse()?,
            comma: input.parse()?,
            literal: input.parse()?,
        })
    }
}

pub struct IncludeInput {
    pub ty: WCharType,
    pub comma: Token![,],
    pub file_path: LitStr,
}

impl Parse for IncludeInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(IncludeInput {
            ty: input.parse()?,
            comma: input.parse()?,
            file_path: input.parse()?,
        })
    }
}
