#![cfg_attr(feature = "unstable", feature(proc_macro_span))]

extern crate proc_macro;

use std::fs;
use std::path::PathBuf;

use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{Error, Result};

use crate::encode::Encode;
use crate::parse::{IncludeInput, LitStrOrChar, WchInput, WchzInput};

mod encode;
mod parse;

// Utility function to handle expanding syn errors into a TokenStream.
fn expand_macro<F: FnOnce() -> Result<TokenStream>>(f: F) -> proc_macro::TokenStream {
    match f() {
        Ok(expanded) => expanded.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

#[proc_macro]
pub fn wchar_t(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let _: syn::parse::Nothing = syn::parse_macro_input!(input);

    let ty = libc::wchar_t::wchar_type();
    ty.to_token_stream().into()
}

#[proc_macro]
pub fn wch(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let WchInput { ty, literal, .. } = syn::parse_macro_input!(input);

    expand_macro(|| {
        let ty = ty.map(|(ty, _)| ty);

        match literal {
            LitStrOrChar::Str(lit) => Ok(encode::expand_str(ty, &lit.value())),
            LitStrOrChar::Char(lit) => encode::expand_char(ty, lit),
        }
    })
}

#[proc_macro]
pub fn wchz(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let WchzInput { ty, literal, .. } = syn::parse_macro_input!(input);

    expand_macro(|| {
        let ty = ty.map(|(ty, _)| ty);
        let text = literal.value();

        if text.as_bytes().contains(&0) {
            return Err(Error::new(
                literal.span(),
                "C-style string cannot contain nul characters",
            ));
        }

        Ok(encode::expand_str_c(ty, &text))
    })
}

#[proc_macro]
pub fn include_wch(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let IncludeInput { ty, file_path, .. } = syn::parse_macro_input!(input);

    expand_macro(|| {
        let ty = ty.map(|(ty, _)| ty);
        let text = read_file(&file_path)?;

        Ok(encode::expand_str(ty, &text))
    })
}

#[proc_macro]
pub fn include_wchz(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let IncludeInput { ty, file_path, .. } = syn::parse_macro_input!(input);

    expand_macro(|| {
        let ty = ty.map(|(ty, _)| ty);
        let text = read_file(&file_path)?;

        if text.as_bytes().contains(&0) {
            return Err(Error::new(
                file_path.span(),
                "C-style string cannot contain nul characters",
            ));
        }

        Ok(encode::expand_str_c(ty, &text))
    })
}

fn read_file(path: &syn::LitStr) -> Result<String> {
    let span = path.span();
    let mut path = PathBuf::from(path.value());

    // If the path is relative, resolve it relative source file directory.
    if path.is_relative() {
        // Get the directory containing the call site source file.
        let mut dir = call_site_dir(span)?;

        // Resolve path relative to dir.
        dir.push(path);
        path = dir;
    }

    match fs::read_to_string(&path) {
        Ok(text) => Ok(text),
        Err(err) => Err(Error::new(
            span,
            format_args!("couldn't read {}: {}", path.display(), err),
        )),
    }
}

// `Span::source()` and `Span::source_file()` are currently unstable.
#[cfg(feature = "unstable")]
fn call_site_dir(_span: Span) -> Result<PathBuf> {
    let call_site = Span::call_site().unwrap().source();
    let source_file = call_site.source_file();

    // The path to the source file.
    let mut path = source_file.path();
    // The path to the directory containing the source file.
    path.pop();

    Ok(path)
}

#[cfg(not(feature = "unstable"))]
fn call_site_dir(span: Span) -> Result<PathBuf> {
    Err(Error::new(
        span,
        "including files by relative path requires the `unstable` feature",
    ))
}
