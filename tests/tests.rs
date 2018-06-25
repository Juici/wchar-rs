#![feature(proc_macro, proc_macro_non_items)]

extern crate wchar;

use wchar::{wch, wch_c};

macro_rules! test_wch {
    ($s:expr) => {
        assert_eq!(wch!($s), &*$s.encode_utf16().collect::<Vec<u16>>())
    };
}

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

#[test]
fn test_wch() {
    test_wch!("foo");
    test_wch!("bar");

    test_wch!("foo bar");

    test_wch!("foo\nbar");
    test_wch!("foo\r\nbar");

    test_wch!("foo\0 bar");
    test_wch!("foo bar\0");
}

#[test]
fn test_wch_c() {
    test_wch_c!("foo");
    test_wch_c!("bar");

    test_wch_c!("foo bar");

    test_wch_c!("foo\nbar");
    test_wch_c!("foo\r\nbar");
}

#[test]
fn test_compare() {
    assert_eq!(wch!("foo\0"), wch_c!("foo"));
    assert_eq!(wch!("bar\0"), wch_c!("bar"));

    assert_eq!(wch!("%HOME%\\foo\\bar\0"), wch_c!(r#"%HOME%\foo\bar"#));
    assert_eq!(wch!("%HOME%\\foo\\bar\\\0"), wch_c!(r#"%HOME%\foo\bar\"#));
}
