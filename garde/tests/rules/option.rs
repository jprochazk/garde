use super::util;

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(required, length(min = 1))]
    v: Option<&'a str>,
}

#[test]
fn option_valid() {
    util::check_ok(&[Test { v: Some("asdf") }], &())
}

#[test]
fn option_invalid() {
    util::check_fail!(&[Test { v: Some("") }, Test { v: None }], &())
}
