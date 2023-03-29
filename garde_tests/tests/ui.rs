#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/compile-fail/**/*.rs");
    t.pass("tests/ui/compile-pass/**/*.rs");
}
