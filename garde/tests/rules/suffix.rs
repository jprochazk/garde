use super::util;
#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(suffix("test"))]
    field: &'a str,
}

#[test]
fn suffix_valid() {
    util::check_ok(&[Test { field: "test" }, Test { field: "asdf_test" }], &())
}

#[test]
fn suffix_invalid() {
    util::check_fail!(&[Test { field: "a" }, Test { field: "test_" }], &())
}
