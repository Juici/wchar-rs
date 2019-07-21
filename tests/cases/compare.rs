use wchar::{wch, wch_c};

fn main() {
    assert_eq!(wch!("foo\0"), wch_c!("foo"));
    assert_eq!(wch!("bar\0"), wch_c!("bar"));

    assert_eq!(wch!("%HOME%\\foo\\bar\0"), wch_c!(r#"%HOME%\foo\bar"#));
    assert_eq!(wch!("%HOME%\\foo\\bar\\\0"), wch_c!(r#"%HOME%\foo\bar\"#));

    assert_eq!(wch!("foo \"bar\" baz\0"), wch_c!(r#"foo "bar" baz"#));
}
