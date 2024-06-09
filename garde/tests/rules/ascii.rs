use std::borrow::Cow;

use super::util;

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(ascii)]
    field: &'a str,

    #[garde(inner(ascii))]
    inner: &'a [&'a str],

    #[garde(ascii)]
    cow: Cow<'a, str>,
}

#[test]
fn ascii_valid() {
    util::check_ok(
        &[Test {
            field: "a!0_~",
            inner: &["a!0_~"],
            cow: Cow::Borrowed("a!0_~"),
        }],
        &(),
    )
}

#[test]
fn ascii_invalid() {
    util::check_fail!(
        &[Test {
            field: "😂",
            inner: &["😂"],
            cow: Cow::Borrowed("😂"),
        }],
        &()
    )
}
