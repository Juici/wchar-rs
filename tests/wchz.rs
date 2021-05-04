use std::iter::once;

use wchar::{wchar_t, wchz};

macro_rules! test_wchz {
    ($s:literal) => {{
        assert_eq!(
            wchz!(u16, $s),
            &*$s.encode_utf16().chain(once(0)).collect::<Vec<_>>()
        );
        assert_eq!(
            wchz!(i16, $s),
            &*$s.encode_utf16()
                .map(|c| c as i16)
                .chain(once(0))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            wchz!(u32, $s),
            &*$s.chars()
                .map(|c| c as u32)
                .chain(once(0))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            wchz!(i32, $s),
            &*$s.chars()
                .map(|c| c as i32)
                .chain(once(0))
                .collect::<Vec<_>>()
        );
    }};
}

// Check we can use the macro to declare constants.
const _: &[wchar_t] = wchz!("const");
const _: &[u16] = wchz!(u16, "const");
const _: &[i16] = wchz!(i16, "const");
const _: &[u32] = wchz!(u32, "const");
const _: &[i32] = wchz!(i32, "const");

#[test]
fn basic() {
    test_wchz!("foo");
    test_wchz!("bar");
    test_wchz!("foo bar");
}

#[test]
fn complex() {
    test_wchz!("京");
    test_wchz!("٣");
    test_wchz!("و");
}

#[test]
fn emoji() {
    test_wchz!("🦀");
    test_wchz!("💖");
}

#[test]
fn escape_chars() {
    test_wchz!("foo\nbar");
    test_wchz!("foo\r\nbar");
    test_wchz!("foo\tbar");
}

#[test]
fn raw_literals() {
    test_wchz!(r"\");
    test_wchz!(r"foo\bar\");

    test_wchz!(r#"foo"bar"#);
    test_wchz!(r#"foo "bar" baz"#);
}
