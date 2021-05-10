#![cfg(feature = "nightly")]

use wchar::{include_wchz, wchar_t};

// Check we can use the macro to declare constants.
const _: &[wchar_t] = include_wchz!("../README.md");
const _: &[u16] = include_wchz!(u16, "../README.md");
const _: &[i16] = include_wchz!(i16, "../README.md");
const _: &[u32] = include_wchz!(u32, "../README.md");
const _: &[i32] = include_wchz!(i32, "../README.md");
