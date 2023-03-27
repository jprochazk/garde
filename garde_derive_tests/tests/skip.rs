#[path = "./util/mod.rs"]
mod util;

#[allow(dead_code)]
#[derive(Debug, garde::Validate)]
struct Test {
    #[garde(skip)]
    field: u64,
}

#[test]
fn skip_valid() {
    util::check_ok(&[Test { field: 50 }], &())
}
