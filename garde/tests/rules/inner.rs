use super::util;

#[derive(Debug, garde::Validate)]
struct Inner<'a> {
    // Double `inner`
    #[garde(inner(inner(alphanumeric)))]
    inner: &'a [&'a [&'a str]],
}

#[test]
fn alphanumeric_valid() {
    util::check_ok(
        &[Inner {
            inner: &[&["abcd0123"]],
        }],
        &(),
    )
}

#[test]
fn alphanumeric_invalid() {
    util::check_fail!(
        &[Inner {
            inner: &[&["!!!!"]]
        }],
        &()
    )
}
