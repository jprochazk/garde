use super::util;
#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(pattern(r"^abcd|efgh$"))]
    field: &'a str,
    #[garde(inner(pattern(r"^abcd|efgh$")))]
    inner: &'a [&'a str],
}

#[test]
fn pattern_valid() {
    util::check_ok(
        &[
            Test {
                field: "abcd",
                inner: &["abcd"],
            },
            Test {
                field: "efgh",
                inner: &["efgh"],
            },
        ],
        &(),
    )
}

#[test]
fn pattern_invalid() {
    util::check_fail!(
        &[
            Test {
                field: "dcba",
                inner: &["dcba"]
            },
            Test {
                field: "hgfe",
                inner: &["hgfe"]
            }
        ],
        &()
    )
}
