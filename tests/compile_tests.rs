#[cfg_attr(not(unix), ignore)]
#[rustversion::attr(not(nightly), ignore)]
#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/*.rs");

    if cfg!(feature = "unstable") {
        t.compile_fail("tests/ui_unstable/*.rs");
    }
}
