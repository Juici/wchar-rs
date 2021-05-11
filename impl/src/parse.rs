use syn::parse::{Parse, ParseStream, Result};
use syn::{Error, Ident, LitStr, Token};

pub enum WCharType {
    U16,
    U32,
    I16,
    I32,
}

impl Parse for WCharType {
    fn parse(input: ParseStream) -> Result<Self> {
        let ty = input.parse::<Ident>()?;
        let s = ty.to_string();
        match &s[..] {
            "u16" => Ok(WCharType::U16),
            "u32" => Ok(WCharType::U32),
            "i16" => Ok(WCharType::I16),
            "i32" => Ok(WCharType::I32),
            _ => Err(Error::new(
                ty.span(),
                format_args!(
                    "unexpected wchar type `{}`, expected one of: u16, u32, i16, i32",
                    s
                ),
            )),
        }
    }
}

pub struct Input {
    pub ty: WCharType,
    pub lit: LitStr,
}

impl Parse for Input {
    fn parse(input: ParseStream) -> Result<Self> {
        let ty = input.parse::<WCharType>()?;
        let _ = input.parse::<Token![,]>()?;
        let lit = input.parse()?;

        Ok(Input { ty, lit })
    }
}
