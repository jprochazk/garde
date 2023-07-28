use super::util;

const TEST: &str = "test";

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(suffix(TEST))]
    field: &'a str,
    #[garde(inner(suffix("test")))]
    inner: &'a [&'a str],
}

#[test]
fn suffix_valid() {
    util::check_ok(
        &[
            Test {
                field: "test",
                inner: &["test"],
            },
            Test {
                field: "asdf_test",
                inner: &["asdf_test"],
            },
        ],
        &(),
    )
}

#[test]
fn suffix_invalid() {
    util::check_fail!(
        &[
            Test {
                field: "a",
                inner: &["a"]
            },
            Test {
                field: "test_",
                inner: &["test_"]
            }
        ],
        &()
    )
}
