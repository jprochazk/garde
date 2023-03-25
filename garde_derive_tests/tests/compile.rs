// TODO: compile tests for tuple structs and enums

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile-fail/**/*.rs");
    t.pass("tests/compile-pass/**/*.rs");
}
