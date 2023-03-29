use super::util;
#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(length(min = 10, max = 100))]
    field: &'a str,
}

#[test]
fn length_valid() {
    util::check_ok(&[
        Test {
            // 'a' * 10
            field: "aaaaaaaaaa",
        },
        Test {
            // 'a' * 100
            field: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        },
    ], &())
}

#[test]
fn length_invalid() {
    util::check_fail!(&[
        Test {
            // 'a' * 9
            field: "aaaaaaaaa",
        },
        Test {
            // 'a' * 101
            field: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        },
    ], &())
}
