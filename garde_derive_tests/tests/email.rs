#[path = "./util/mod.rs"]
mod util;

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(email)]
    field: &'a str,
}

#[test]
fn email_valid() {
    util::check_ok(
        &[Test {
            field: "email@here.com",
        }],
        &(),
    )
}

#[test]
fn email_invalid() {
    util::check_fail!(
        &[Test {
            field: "invalid.com"
        }],
        &()
    )
}
