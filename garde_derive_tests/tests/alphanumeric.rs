#[path = "./util/mod.rs"]
mod util;

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(alphanumeric)]
    field: &'a str,
}

#[test]
fn alphanumeric_valid() {
    util::check_ok(&[Test { field: "abcd0123" }], &())
}

#[test]
fn alphanumeric_invalid() {
    util::check_fail!(&[Test { field: "!!!!" }], &())
}
