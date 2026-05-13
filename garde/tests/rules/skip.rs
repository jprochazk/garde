use super::util;

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

#[derive(Debug, garde::Validate)]
struct Inner(#[garde(ascii)] String);

#[allow(dead_code)]
#[derive(Debug, garde::Validate)]
enum SkipBeforeDive {
    Variant(#[garde(skip)] u64, #[garde(dive)] Inner),
}

#[test]
fn skip_valid() {
    util::check_ok(&[Struct { field: 50 }], &());
    util::check_ok(&[Tuple(50)], &());
    util::check_ok(&[Enum::Struct { field: 50 }, Enum::Tuple(50)], &());
    util::check_ok(&[SkipBeforeDive::Variant(1, Inner("ascii".into()))], &());
}

#[test]
fn skip_before_dive_validates_correct_field() {
    use garde::Validate;
    assert!(SkipBeforeDive::Variant(1, Inner("not ascii: 😀".into()))
        .validate()
        .is_err());
}
