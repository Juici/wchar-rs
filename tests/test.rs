#[test]
fn test() {
    let t = trybuild::TestCases::new();

    t.pass("tests/cases/wch.rs");
    t.pass("tests/cases/wch_c.rs");

    t.pass("tests/cases/compare.rs");

    t.compile_fail("tests/cases/nul-compile-fail-1.rs");
    t.compile_fail("tests/cases/nul-compile-fail-2.rs");
}
