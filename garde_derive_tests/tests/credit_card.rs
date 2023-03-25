#[path = "./util/mod.rs"]
mod util;

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(credit_card)]
    field: &'a str,
}

#[test]
fn credit_card_valid() {
    util::check_ok(
        &[
            Test {
                field: "4539571147647251",
            },
            Test {
                field: "343380440754432",
            },
        ],
        &(),
    )
}

#[test]
fn credit_card_invalid() {
    util::check_fail!(
        &[
            Test {
                field: "zduhefljsdfKJKJZHUI"
            },
            Test {
                field: "5236313877109141"
            },
        ],
        &()
    )
}
