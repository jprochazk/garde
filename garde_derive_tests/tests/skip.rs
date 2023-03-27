#[path = "./util/mod.rs"]
mod util;

#[allow(dead_code)]
#[derive(Debug, garde::Validate)]
struct Struct {
    #[garde(skip)]
    field: u64,
}

#[allow(dead_code)]
#[derive(Debug, garde::Validate)]
struct Tuple(#[garde(skip)] u64);

#[allow(dead_code)]
#[derive(Debug, garde::Validate)]
enum Enum {
    Struct {
        #[garde(skip)]
        field: u64,
    },
    Tuple(#[garde(skip)] u64),
}

#[test]
fn skip_valid() {
    util::check_ok(&[Struct { field: 50 }], &());
    util::check_ok(&[Tuple(50)], &());
    util::check_ok(&[Enum::Struct { field: 50 }, Enum::Tuple(50)], &());
}
