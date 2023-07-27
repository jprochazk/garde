use super::util;

#[allow(dead_code)]
#[derive(Debug, garde::Validate)]
#[garde(allow_unvalidated)]
struct Test<'a> {
    #[garde(ascii)]
    field: &'a str,

    unvalidated: &'a str,
}

#[test]
fn ascii_valid() {
    util::check_ok(
        &[Test {
            field: "a!0_~",
            unvalidated: "",
        }],
        &(),
    )
}

#[test]
fn ascii_invalid() {
    util::check_fail!(
        &[Test {
            field: "😂",
            unvalidated: "",
        }],
        &()
    )
}
