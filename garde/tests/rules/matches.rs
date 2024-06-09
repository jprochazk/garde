use super::util;

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(skip)]
    foo: &'a str,

    #[garde(matches(foo))]
    bar: &'a str,

    #[garde(inner(matches(foo)))]
    inner: &'a [&'a str],
}

#[test]
fn matches_valid() {
    util::check_ok(
        &[Test {
            foo: "_test_",
            bar: "_test_",
            inner: &["_test_"],
        }],
        &(),
    )
}

#[test]
fn matches_invalid() {
    util::check_fail!(
        &[Test {
            foo: "_test_",
            bar: "_test",
            inner: &["_test"],
        }],
        &(),
    )
}
