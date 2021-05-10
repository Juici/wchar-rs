#![cfg(feature = "nightly")]

use wchar::{include_wch, wchar_t};

// Check we can use the macro to declare constants.
const _: &[wchar_t] = include_wch!("../README.md");
const _: &[u16] = include_wch!(u16, "../README.md");
const _: &[i16] = include_wch!(i16, "../README.md");
const _: &[u32] = include_wch!(u32, "../README.md");
const _: &[i32] = include_wch!(i32, "../README.md");
