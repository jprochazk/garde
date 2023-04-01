use super::util;

#[derive(Debug, garde::Validate)]
struct Inner<'a> {
    #[garde(length(min = 1))]
    field: &'a str,
}

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(dive, length(min = 1))]
    field: Vec<Inner<'a>>,
}

#[test]
fn dive_with_rules_valid() {
    util::check_ok(
        &[Test {
            field: vec![Inner { field: "asdf" }],
        }],
        &(),
    )
}
#[test]
fn dive_with_rules_invalid() {
    util::check_fail!(
        &[
            Test { field: vec![] },
            Test {
                field: vec![Inner { field: "" }]
            }
        ],
        &(),
    )
}
