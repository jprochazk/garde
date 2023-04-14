use super::util;
#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(contains("test"))]
    field: &'a str,
}

#[test]
fn contains_valid() {
    util::check_ok(&[Test { field: "_test_" }], &())
}

#[test]
fn contains_invalid() {
    util::check_fail!(&[Test { field: "_____" }], &())
}
