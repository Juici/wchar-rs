use wchar::{wch, wchz};

macro_rules! compare {
    ($s1:literal, $s2:literal) => {{
        assert_eq!(wch!($s1), wchz!($s2));
        assert_eq!(wch!(u16, $s1), wchz!(u16, $s2));
        assert_eq!(wch!(i16, $s1), wchz!(i16, $s2));
        assert_eq!(wch!(u32, $s1), wchz!(u32, $s2));
        assert_eq!(wch!(i32, $s1), wchz!(i32, $s2));
    }};
}

fn main() {
    compare!("foo\0", "foo");
    compare!("bar\0", "bar");

    compare!("%HOME%\\foo\\bar\0", r#"%HOME%\foo\bar"#);
    compare!("%HOME%\\foo\\bar\\\0", r#"%HOME%\foo\bar\"#);

    compare!("foo \"bar\" baz\0", r#"foo "bar" baz"#);
}
