use super::util;
#[derive(Debug, garde::Validate)]
struct Test {
    #[garde(range(min = 10, max = 100))]
    field: u64,
}

#[test]
fn range_valid() {
    util::check_ok(&[Test { field: 50 }], &())
}

#[test]
fn range_invalid() {
    util::check_fail!(&[Test { field: 9 }, Test { field: 101 }], &())
}
