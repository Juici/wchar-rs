use std::iter::once;

use wchar::wch_c;

macro_rules! test_wch_c {
    ($s:literal) => {{
        assert_eq!(wch_c!($s), wch_c!(u16, $s));
        assert_eq!(
            wch_c!(u16, $s),
            &*$s.encode_utf16().chain(once(0)).collect::<Vec<_>>()
        );
        assert_eq!(
            wch_c!(i16, $s),
            &*$s.encode_utf16()
                .map(|c| c as i16)
                .chain(once(0))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            wch_c!(u32, $s),
            &*$s.chars()
                .map(|c| c as u32)
                .chain(once(0))
                .collect::<Vec<_>>()
        );
        assert_eq!(
            wch_c!(i32, $s),
            &*$s.chars()
                .map(|c| c as i32)
                .chain(once(0))
                .collect::<Vec<_>>()
        );
    }};
}

// Check we can use the macro to declare constants.
const _: &[u16] = wch_c!("const");
const _: &[u16] = wch_c!(u16, "const");
const _: &[i16] = wch_c!(i16, "const");
const _: &[u32] = wch_c!(u32, "const");
const _: &[i32] = wch_c!(i32, "const");

#[test]
fn basic() {
    test_wch_c!("foo");
    test_wch_c!("bar");
    test_wch_c!("foo bar");
}

#[test]
fn escape_chars() {
    test_wch_c!("foo\nbar");
    test_wch_c!("foo\r\nbar");
    test_wch_c!("foo\tbar");
}

#[test]
fn raw_literals() {
    test_wch_c!(r"\");
    test_wch_c!(r"foo\bar\");

    test_wch_c!(r#"foo"bar"#);
    test_wch_c!(r#"foo "bar" baz"#);
}
