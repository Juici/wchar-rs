use wchar::{wch, wchar_t};

mod util;

macro_rules! test_wch {
    ($s:literal) => {{
        use util::Wide;

        let string = $s;

        let v = wch!(u16, $s);
        assert_eq!(v, &*u16::encode_str(string));
        assert_eq!(string, u16::decode_str(v.into_iter().copied()).unwrap());

        let v = wch!(u32, $s);
        assert_eq!(v, &*u32::encode_str(string));
        assert_eq!(string, u32::decode_str(v.into_iter().copied()).unwrap());

        let v = wch!(i16, $s);
        assert_eq!(v, &*i16::encode_str(string));
        assert_eq!(string, i16::decode_str(v.into_iter().copied()).unwrap());

        let v = wch!(i32, $s);
        assert_eq!(v, &*i32::encode_str(string));
        assert_eq!(string, i32::decode_str(v.into_iter().copied()).unwrap());
    }};
}

// Check we can use the macro to declare constants.
const _: &[wchar_t] = wch!("const");
const _: &[u16] = wch!(u16, "const");
const _: &[u32] = wch!(u32, "const");
const _: &[i16] = wch!(i16, "const");
const _: &[i32] = wch!(i32, "const");

#[test]
fn basic() {
    test_wch!("foo");
    test_wch!("bar");
    test_wch!("foo bar");
}

#[test]
fn complex() {
    test_wch!("京");
    test_wch!("٣");
    test_wch!("و");
    test_wch!("𐐷");
}

#[test]
fn emoji() {
    test_wch!("🦀");
    test_wch!("💖");
    test_wch!("🇬🇧");
}

#[test]
fn escape_chars() {
    test_wch!("foo\nbar");
    test_wch!("foo\r\nbar");
    test_wch!("foo\tbar");
}

#[test]
fn nul_chars() {
    test_wch!("foo\0bar");
    test_wch!("foo bar\0");
}

#[test]
fn raw_literals() {
    test_wch!(r"\");
    test_wch!(r"foo\bar\");

    test_wch!(r#"foo"bar"#);
    test_wch!(r#"foo "bar" baz"#);
}
