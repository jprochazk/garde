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

#[derive(Debug, garde::Validate)]
struct NotNestedOption<'a> {
    #[garde(inner(alphanumeric))]
    inner: Option<&'a str>,
}

#[derive(Debug, garde::Validate)]
struct NestedSliceInsideOption<'a> {
    #[garde(inner(inner(alphanumeric)))]
    inner: Option<&'a [&'a str]>,
}

#[derive(Debug, garde::Validate)]
struct DoubleNestedSliceInsideOption<'a> {
    #[garde(inner(inner(inner(alphanumeric))))]
    inner: Option<&'a [&'a [&'a str]]>,
}

#[derive(Debug, garde::Validate)]
struct OptionInsideSlice<'a> {
    #[garde(inner(inner(alphanumeric)))]
    inner: &'a [Option<&'a str>],
}

#[test]
fn alphanumeric_some_valid() {
    util::check_ok(
        &[NotNestedOption {
            inner: Some("abcd0123"),
        }],
        &(),
    );
    util::check_ok(
        &[NestedSliceInsideOption {
            inner: Some(&["abcd0123"]),
        }],
        &(),
    );
    util::check_ok(
        &[DoubleNestedSliceInsideOption {
            inner: Some(&[&["abcd0123"]]),
        }],
        &(),
    );
    util::check_ok(
        &[OptionInsideSlice {
            inner: &[Some("abcd0123")],
        }],
        &(),
    )
}

#[test]
fn alphanumeric_some_invalid() {
    util::check_fail!(
        &[NotNestedOption {
            inner: Some("!!!!"),
        }],
        &(),
    );
    util::check_fail!(
        &[NestedSliceInsideOption {
            inner: Some(&["!!!!"]),
        }],
        &(),
    );
    util::check_fail!(
        &[DoubleNestedSliceInsideOption {
            inner: Some(&[&["!!!!"]]),
        }],
        &(),
    );
    util::check_fail!(
        &[OptionInsideSlice {
            inner: &[Some("!!!!")],
        }],
        &(),
    )
}

#[test]
fn alphanumeric_none_valid() {
    util::check_ok(&[NotNestedOption { inner: None }], &());
    util::check_ok(&[NestedSliceInsideOption { inner: None }], &());
    util::check_ok(&[DoubleNestedSliceInsideOption { inner: None }], &());
    util::check_ok(
        &[OptionInsideSlice {
            inner: &[None, None],
        }],
        &(),
    )
}
