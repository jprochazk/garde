use super::util;

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(prefix("test"), ascii, length(min = 10, max = 100))]
    field: &'a str,
    #[garde(inner(prefix("test"), ascii, length(min = 10, max = 100)))]
    inner: &'a [&'a str],
}

#[test]
fn multi_rule_valid() {
    util::check_ok(
        &[
            Test {
                field: "test_test_test",
                inner: &["test_test_test"]
            },
            Test {
                field: "test_some_other_ascii_but_less_than_100_chars",
                inner: &["test_some_other_ascii_but_less_than_100_chars"]
            },
            Test {
                // "test" + 'a' * 6
                field: "testaaaaaa",
                inner: &["testaaaaaa"]
            },
            Test {
                // "test" + 'a' * 96
                field: "testaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                inner: &["testaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"]
            },
        ],
        &(),
    )
}

#[test]
fn multi_rule_invalid() {
    util::check_fail!(
        &[
            Test {
                field: "text which does not begin with `test`",
                inner: &["text which does not begin with `test`"],
            },
            Test {
                field: "non-ascii 😂😂😂",
                inner: &["non-ascii 😂😂😂"],
            },
            Test {
                // 'a' * 9
                field: "aaaaaaaaa",
                inner: &["aaaaaaaaa"],
            },
            Test {
                // 'a' * 101
                field: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                inner: &["aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"]
            },
        ],
        &()
    )
}
