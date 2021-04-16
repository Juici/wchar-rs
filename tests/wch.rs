use wchar::wch;

macro_rules! test_wch {
    ($s:literal) => {{
        assert_eq!(wch!($s), wch!(u16, $s));
        assert_eq!(wch!(u16, $s), &*$s.encode_utf16().collect::<Vec<_>>());
        assert_eq!(
            wch!(i16, $s),
            &*$s.encode_utf16().map(|c| c as i16).collect::<Vec<_>>()
        );
        assert_eq!(
            wch!(u32, $s),
            &*$s.chars().map(|c| c as u32).collect::<Vec<_>>()
        );
        assert_eq!(
            wch!(i32, $s),
            &*$s.chars().map(|c| c as i32).collect::<Vec<_>>()
        );
    }};
}

// Check we can use the macro to declare constants.
const _: &[u16] = wch!("const");
const _: &[u16] = wch!(u16, "const");
const _: &[i16] = wch!(i16, "const");
const _: &[u32] = wch!(u32, "const");
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
}

#[test]
fn emoji() {
    test_wch!("🦀");
    test_wch!("💖");
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
