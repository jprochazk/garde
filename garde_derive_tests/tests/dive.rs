#[path = "./util/mod.rs"]
mod util;

#[derive(Debug, garde::Validate)]
struct Inner<'a> {
    #[garde(length(min = 1))]
    field: &'a str,
}

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(dive)]
    field: Inner<'a>,
}

#[test]
fn email_valid() {
    util::check_ok(
        &[Test {
            field: Inner { field: "asdf" },
        }],
        &(),
    )
}

#[test]
fn email_invalid() {
    util::check_fail!(
        &[Test {
            field: Inner { field: "" }
        }],
        &()
    )
}
