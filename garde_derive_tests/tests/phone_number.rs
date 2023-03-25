#[path = "./util/mod.rs"]
mod util;

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(phone_number)]
    field: &'a str,
}

#[test]
fn phone_number_valid() {
    util::check_ok(
        &[
            Test {
                field: "+1 (415) 237-0800",
            },
            Test {
                field: "+14152370800",
            },
            Test {
                field: "+33642926829",
            },
        ],
        &(),
    )
}

#[test]
fn phone_number_invalid() {
    util::check_fail!(
        &[
            Test {
                field: "14152370800"
            },
            Test {
                field: "0642926829"
            },
            Test {
                field: "00642926829"
            },
            Test { field: "A012" },
            Test { field: "TEXT" },
        ],
        &()
    )
}
