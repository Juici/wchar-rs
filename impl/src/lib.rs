#![cfg_attr(feature = "nightly", feature(proc_macro_span))]

extern crate proc_macro;

use crate::parse::Input;

mod encode;
mod parse;

#[proc_macro]
pub fn wch(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Input { ty, lit } = syn::parse_macro_input!(input);

    let text = lit.value();

    encode::expand(ty, &text).into()
}

#[proc_macro]
pub fn wchz(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Input { ty, lit } = syn::parse_macro_input!(input);

    let text = lit.value();
    if text.as_bytes().contains(&0) {
        return syn::Error::new(lit.span(), "C-style string cannot contain nul characters")
            .to_compile_error()
            .into();
    }

    encode::expand_c(ty, &text).into()
}

#[cfg(feature = "nightly")]
#[proc_macro]
pub fn include_wch(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Input { ty, lit } = syn::parse_macro_input!(input);

    let text = match read_file(&lit) {
        Ok(text) => text,
        Err(err) => return err,
    };

    encode::expand(ty, &text).into()
}

#[cfg(feature = "nightly")]
#[proc_macro]
pub fn include_wchz(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Input { ty, lit } = syn::parse_macro_input!(input);

    let text = match read_file(&lit) {
        Ok(text) => text,
        Err(err) => return err,
    };

    if text.as_bytes().contains(&0) {
        return syn::Error::new(lit.span(), "C-style string cannot contain nul characters")
            .to_compile_error()
            .into();
    }

    encode::expand_c(ty, &text).into()
}

#[cfg(feature = "nightly")]
fn read_file(lit: &syn::LitStr) -> Result<String, proc_macro::TokenStream> {
    let call_site = proc_macro::Span::call_site().source();
    let source_file = call_site.source_file();

    let mut path = source_file.path();
    path.pop();
    path.push(lit.value());

    match std::fs::read_to_string(&path) {
        Ok(text) => Ok(text),
        Err(err) => Err(syn::Error::new(
            lit.span(),
            format_args!("couldn't read {}: {}", path.display(), err),
        )
        .to_compile_error()
        .into()),
    }
}
