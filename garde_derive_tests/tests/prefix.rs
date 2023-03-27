#[path = "./util/mod.rs"]
mod util;

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(prefix("test"))]
    field: &'a str,
}

#[test]
fn prefix_valid() {
    util::check_ok(&[Test { field: "test" }, Test { field: "test_asdf" }], &())
}

#[test]
fn prefix_invalid() {
    util::check_fail!(&[Test { field: "a" }, Test { field: "_test" }], &())
}
