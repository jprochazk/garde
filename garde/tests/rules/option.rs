use super::util;

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(required, length(min = 1))]
    a: Option<&'a str>,
    #[garde(length(min = 1))]
    b: Option<&'a str>,
}

#[test]
fn option_valid() {
    util::check_ok(
        &[Test {
            a: Some("asdf"),
            b: Some("asdf"),
        }],
        &(),
    )
}

#[test]
fn option_invalid() {
    util::check_fail!(
        &[
            Test {
                a: Some(""),
                b: Some("")
            },
            Test { a: None, b: None }
        ],
        &()
    )
}
