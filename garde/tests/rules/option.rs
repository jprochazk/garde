use super::util;

#[derive(Debug, garde::Validate)]
struct Test<'a> {
    #[garde(alphanumeric)]
    alphanumeric: Option<&'a str>,
    #[garde(ascii)]
    ascii: Option<&'a str>,
    #[garde(byte_length(min = 1))]
    byte_length_min1_str: Option<&'a str>,
    #[garde(byte_length(min = 1))]
    byte_length_min1_u8_slice: Option<&'a [u8]>,
    #[garde(contains("a"))]
    contains_a: Option<&'a str>,
    #[garde(credit_card)]
    credit_card: Option<&'a str>,
    #[garde(email)]
    email: Option<&'a str>,
    #[garde(ip)]
    ip: Option<&'a str>,
    #[garde(length(min = 1))]
    length_min1: Option<&'a str>,
    #[garde(pattern(r"a|b"))]
    pat_a_or_b: Option<&'a str>,
    #[garde(phone_number)]
    phone_number: Option<&'a str>,
    #[garde(prefix("a"))]
    prefix_a: Option<&'a str>,
    #[garde(range(min = 1))]
    range_min1: Option<i32>,
    #[garde(required)]
    required: Option<&'a str>,
    #[garde(suffix("a"))]
    suffix_a: Option<&'a str>,
    #[garde(url)]
    url: Option<&'a str>,
}

#[test]
fn option_valid() {
    util::check_ok(
        &[Test {
            alphanumeric: Some("a"),
            ascii: Some("a"),
            byte_length_min1_str: Some("a"),
            byte_length_min1_u8_slice: Some(&[0]),
            contains_a: Some("a"),
            credit_card: Some("4539571147647251"),
            email: Some("test@mail.com"),
            ip: Some("127.0.0.1"),
            length_min1: Some("a"),
            pat_a_or_b: Some("a"),
            phone_number: Some("+14152370800"),
            prefix_a: Some("a"),
            range_min1: Some(1),
            required: Some("a"),
            suffix_a: Some("a"),
            url: Some("http://test.com"),
        }],
        &(),
    )
}

#[test]
fn option_invalid() {
    util::check_fail!(
        &[
            Test {
                alphanumeric: Some("😂"),
                ascii: Some("😂"),
                byte_length_min1_str: Some(""),
                byte_length_min1_u8_slice: Some(&[]),
                contains_a: Some("😂"),
                credit_card: Some("😂"),
                email: Some("😂"),
                ip: Some("😂"),
                length_min1: Some(""),
                pat_a_or_b: Some("😂"),
                phone_number: Some("😂"),
                prefix_a: Some(""),
                range_min1: Some(0),
                required: None,
                suffix_a: Some("😂"),
                url: Some("😂"),
            },
            Test {
                alphanumeric: None,
                ascii: None,
                byte_length_min1_str: None,
                byte_length_min1_u8_slice: None,
                contains_a: None,
                credit_card: None,
                email: None,
                ip: None,
                length_min1: None,
                pat_a_or_b: None,
                phone_number: None,
                prefix_a: None,
                range_min1: None,
                required: None,
                suffix_a: None,
                url: None,
            }
        ],
        &()
    )
}
