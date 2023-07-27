use super::util;
#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(prefix("test"))]
    field: &'a str,
    #[garde(inner(prefix("test")))]
    inner: &'a [&'a str],
}

#[test]
fn prefix_valid() {
    util::check_ok(
        &[
            Test {
                field: "test",
                inner: &["test"],
            },
            Test {
                field: "test_asdf",
                inner: &["test_asdf"],
            },
        ],
        &(),
    )
}

#[test]
fn prefix_invalid() {
    util::check_fail!(
        &[
            Test {
                field: "a",
                inner: &["a"]
            },
            Test {
                field: "_test",
                inner: &["_test"]
            }
        ],
        &()
    )
}
