#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");

    if cfg!(feature = "nightly") {
        t.compile_fail("tests/ui_unstable/*.rs");
    }
}
