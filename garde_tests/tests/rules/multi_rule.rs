use super::util;
#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(prefix("test"), ascii, length(min = 10, max = 100))]
    field: &'a str,
}

#[test]
fn multi_rule_valid() {
    util::check_ok(
        &[
            Test {
                field: "test_test_test",
            },
            Test {
                field: "test_some_other_ascii_but_less_than_100_chars",
            },
            Test {
                // "test" + 'a' * 6
                field: "testaaaaaa",
            },
            Test {
                // "test" + 'a' * 96
                field: "testaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
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
            },
            Test {
                field: "non-ascii 😂😂😂"
            },
            Test {
                // 'a' * 9
                field: "aaaaaaaaa",
            },
            Test {
                // 'a' * 101
                field: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            },
        ],
        &()
    )
}
