use wchar::{wchar_t, wchz};

mod util;

macro_rules! test_wchz {
    ($s:literal) => {{
        use util::Wide;

        let string = $s;

        let v = wchz!(u16, $s);
        assert_eq!(v, &*u16::encode_wide_c(string));
        assert_eq!(string, u16::decode_wide_c(v.into_iter().copied()).unwrap());

        let v = wchz!(u32, $s);
        assert_eq!(v, &*u32::encode_wide_c(string));
        assert_eq!(string, u32::decode_wide_c(v.into_iter().copied()).unwrap());

        let v = wchz!(i16, $s);
        assert_eq!(v, &*i16::encode_wide_c(string));
        assert_eq!(string, i16::decode_wide_c(v.into_iter().copied()).unwrap());

        let v = wchz!(i32, $s);
        assert_eq!(v, &*i32::encode_wide_c(string));
        assert_eq!(string, i32::decode_wide_c(v.into_iter().copied()).unwrap());
    }};
}

// Check we can use the macro to declare constants.
const _: &[wchar_t] = wchz!("const");
const _: &[u16] = wchz!(u16, "const");
const _: &[u32] = wchz!(u32, "const");
const _: &[i16] = wchz!(i16, "const");
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
    test_wchz!("🇬🇧");
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
