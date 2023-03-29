use super::util;
#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(byte_length(min = 10, max = 100))]
    field: &'a str,
}

#[test]
fn byte_length_valid() {
    util::check_ok(&[
        Test {
            // 'a' * 10
            field: "aaaaaaaaaa",
        },
        Test {
            // 'a' * 100
            field: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        },
        Test {
            // "😂" = 4 bytes
            // "😂" * 25 = 100 bytes
            field: "😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂",
        },
    ], &())
}

#[test]
fn byte_length_invalid() {
    util::check_fail!(&[
        Test {
            // 'a' * 9
            field: "aaaaaaaaa",
        },
        Test {
            // 'a' * 101
            field: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        },
        Test {
            // "😂" = 4 bytes
            // 'a' * 1 + "😂" * 25 = 101 bytes
            field: "a😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂😂",
        },
    ], &())
}
