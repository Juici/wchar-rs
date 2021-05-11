#![cfg(feature = "nightly")]

mod util;

use util::Wide;
use wchar::{include_wch, wchar_t};

// Check we can use the macro to declare constants.
const _: &[wchar_t] = include_wch!("../README.md");
const _: &[u16] = include_wch!(u16, "../README.md");
const _: &[i16] = include_wch!(i16, "../README.md");
const _: &[u32] = include_wch!(u32, "../README.md");
const _: &[i32] = include_wch!(i32, "../README.md");

macro_rules! test_include_wch {
    ($file:literal) => {{
        let string = include_str!($file);

        let v = include_wch!(u16, $file);
        assert_eq!(v, &*u16::encode_wide(string));
        assert_eq!(string, u16::decode_wide(v.into_iter().copied()).unwrap());

        let v = include_wch!(u32, $file);
        assert_eq!(v, &*u32::encode_wide(string));
        assert_eq!(string, u32::decode_wide(v.into_iter().copied()).unwrap());

        let v = include_wch!(i16, $file);
        assert_eq!(v, &*i16::encode_wide(string));
        assert_eq!(string, i16::decode_wide(v.into_iter().copied()).unwrap());

        let v = include_wch!(i32, $file);
        assert_eq!(v, &*i32::encode_wide(string));
        assert_eq!(string, i32::decode_wide(v.into_iter().copied()).unwrap());
    }};
}

macro_rules! tests {
    ($($name:ident: $file:literal;)*) => {
        $(
            #[test]
            fn $name() {
                test_include_wch!($file);
            }
        )*
    };
}

tests! {
    basic: "data/basic.txt";
    complex: "data/complex.txt";
    emoji: "data/emoji.txt";
    nul_chars: "data/nul_chars.txt";
}
