use super::util;

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(phone_number)]
    field: &'a str,
    #[garde(inner(phone_number))]
    inner: &'a [&'a str],
}

#[test]
fn phone_number_valid() {
    util::check_ok(
        &[
            Test {
                field: "+1 (415) 237-0800",
                inner: &["+1 (415) 237-0800"],
            },
            Test {
                field: "+14152370800",
                inner: &["+14152370800"],
            },
            Test {
                field: "+33642926829",
                inner: &["+33642926829"],
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
                field: "14152370800",
                inner: &["14152370800"]
            },
            Test {
                field: "0642926829",
                inner: &["0642926829"]
            },
            Test {
                field: "00642926829",
                inner: &["00642926829"]
            },
            Test {
                field: "A012",
                inner: &["A012"]
            },
            Test {
                field: "TEXT",
                inner: &["TEXT"]
            },
        ],
        &()
    )
}
