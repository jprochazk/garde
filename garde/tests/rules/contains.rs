use super::util;

const TEST: &str = "test";

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(contains("test"))]
    field: &'a str,

    #[garde(contains(TEST))]
    field_path: &'a str,

    #[garde(contains(&format!("{}{}", "te", "st")))]
    field_call: &'a str,

    #[garde(inner(contains("test")))]
    inner: &'a [&'a str],
}

#[test]
fn contains_valid() {
    util::check_ok(
        &[Test {
            field: "_test_",
            field_path: "_test_",
            field_call: "_test_",
            inner: &["_test_"],
        }],
        &(),
    )
}

#[test]
fn contains_invalid() {
    util::check_fail!(
        &[Test {
            field: "_____",
            field_path: "_____",
            field_call: "_____",
            inner: &["_____"]
        }],
        &()
    )
}
