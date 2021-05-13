use wchar::{wch, wchar_t};

mod util;

// Characters that fit in a single UTF-16 or UTF-32 codepoint.
macro_rules! test_small {
    ($c:literal) => {{
        use util::Wide;

        let c = $c;

        let v = wch!(u16, $c);
        assert_eq!(v, u16::encode_char(c).unwrap());
        assert_eq!(c, u16::decode_char(v).unwrap());

        let v = wch!(i16, $c);
        assert_eq!(v, i16::encode_char(c).unwrap());
        assert_eq!(c, i16::decode_char(v).unwrap());

        test_large!($c);
    }};
}
// Characters that fit in a single UTF-32 codepoint, but not UTF-16.
macro_rules! test_large {
    ($c:literal) => {{
        use util::Wide;

        let c = $c;

        let v = wch!(u32, $c);
        assert_eq!(v, u32::encode_char(c).unwrap());
        assert_eq!(c, u32::decode_char(v).unwrap());

        let v = wch!(i32, $c);
        assert_eq!(v, i32::encode_char(c).unwrap());
        assert_eq!(c, i32::decode_char(v).unwrap());
    }};
}

// Check we can use the macro to declare constants.
const _: wchar_t = wch!('A');
const _: u16 = wch!(u16, 'A');
const _: u32 = wch!(u32, 'A');
const _: i16 = wch!(i16, 'A');
const _: i32 = wch!(i32, 'A');

// Check characters that overflow UTF-16 but not UTF-32.
const _: u32 = wch!(u32, '💖');
const _: i32 = wch!(i32, '💖');

#[test]
fn basic() {
    test_small!('A');
    test_small!('a');
    test_small!('b');
}

#[test]
fn complex() {
    test_small!('京');
    test_small!('٣');
    test_small!('و');

    test_large!('𐐷');
}

#[test]
fn emoji() {
    test_large!('🦀');
    test_large!('💖');
}

#[test]
fn escape_chars() {
    test_small!('\n');
    test_small!('\r');
    test_small!('\t');
    test_small!('\0');
    test_small!('\x7F');
    test_small!('\u{0259}');

    test_large!('\u{1F980}');
}
