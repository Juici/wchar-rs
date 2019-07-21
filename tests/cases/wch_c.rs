use wchar::wch_c;

macro_rules! test_wch_c {
    ($s:expr) => {
        assert_eq!(
            wch_c!($s),
            &*$s.encode_utf16()
                .chain(std::iter::once(0))
                .collect::<Vec<u16>>()
        )
    };
}

fn main() {
    test_wch_c!("foo");
    test_wch_c!("bar");

    test_wch_c!("foo bar");

    test_wch_c!("foo\nbar");
    test_wch_c!("foo\r\nbar");

    test_wch_c!(r#"foo\bar\"#);
    test_wch_c!(r#"foo "bar" baz"#);

}
