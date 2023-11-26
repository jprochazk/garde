use super::util;

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(email)]
    field: &'a str,
    #[garde(inner(email))]
    inner: &'a [&'a str],
}

#[test]
fn email_valid() {
    util::check_ok(
        &[Test {
            field: "email@here.com",
            inner: &["email@here.com"],
        }],
        &(),
    )
}

#[test]
fn email_invalid() {
    util::check_fail!(
        &[Test {
            field: "invalid.com",
            inner: &["invalid.com"],
        }],
        &()
    )
}
