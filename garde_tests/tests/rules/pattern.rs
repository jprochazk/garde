use super::util;
#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(pattern(r"^abcd|efgh$"))]
    field: &'a str,
}

#[test]
fn pattern_valid() {
    util::check_ok(&[Test { field: "abcd" }, Test { field: "efgh" }], &())
}

#[test]
fn pattern_invalid() {
    util::check_fail!(&[Test { field: "dcba" }, Test { field: "hgfe" }], &())
}
