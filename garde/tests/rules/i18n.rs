use super::util;

struct TestI18n;

impl garde::I18n for TestI18n {
    fn length_lower_than(&self, min: usize) -> String {
        format!("custom: too short, need at least {min}")
    }

    fn length_greater_than(&self, max: usize) -> String {
        format!("custom: too long, maximum is {max}")
    }

    fn range_lower_than(&self, min: &str) -> String {
        format!("custom: value too small, minimum is {min}")
    }

    fn range_greater_than(&self, max: &str) -> String {
        format!("custom: value too large, maximum is {max}")
    }

    fn credit_card_invalid(&self, error: &str) -> String {
        format!("custom: invalid credit card - {error}")
    }

    fn pattern_no_match(&self, pattern: &str) -> String {
        format!("custom: doesn't match /{pattern}/")
    }

    fn contains_missing(&self, pattern: &str) -> String {
        format!("custom: missing required text '{pattern}'")
    }

    fn url_invalid(&self, error: &str) -> String {
        format!("custom: bad URL - {error}")
    }

    fn prefix_missing(&self, pattern: &str) -> String {
        format!("custom: must start with '{pattern}'")
    }

    fn suffix_missing(&self, pattern: &str) -> String {
        format!("custom: must end with '{pattern}'")
    }

    fn phone_number_invalid(&self) -> String {
        "custom: invalid phone number".to_string()
    }

    fn phone_number_invalid_with_error(&self, error: &str) -> String {
        format!("custom: phone number error - {error}")
    }

    fn ip_invalid(&self, kind: &str) -> String {
        format!("custom: not a valid {kind} IP")
    }

    fn matches_field_mismatch(&self, field: &str) -> String {
        format!("custom: must match {field}")
    }

    fn email_invalid(&self, error: &str) -> String {
        format!("custom: bad email - {error}")
    }

    fn ascii_invalid(&self) -> String {
        "custom: contains non-ASCII characters".to_string()
    }

    fn alphanumeric_invalid(&self) -> String {
        "custom: only letters and numbers allowed".to_string()
    }

    fn required_not_set(&self) -> String {
        "custom: this field is required".to_string()
    }
}

#[derive(Debug, garde::Validate)]
struct User {
    #[garde(length(min = 3, max = 20))]
    name: String,
    #[garde(email)]
    email: String,
    #[garde(range(min = 18, max = 120))]
    age: u32,
    #[garde(required)]
    phone: Option<String>,
}

#[derive(Debug, garde::Validate)]
struct Address {
    #[garde(length(min = 5))]
    street: String,
    #[garde(length(min = 2))]
    city: String,
    #[garde(pattern(r"^[0-9]{5}$"))]
    zip_code: String,
}

#[derive(Debug, garde::Validate)]
struct Profile {
    #[garde(dive)]
    user: User,
    #[garde(dive)]
    address: Address,
    #[garde(length(min = 10))]
    bio: String,
}

#[test]
fn test_default_i18n_messages() {
    use garde::I18n as _;
    let default = garde::i18n::DefaultI18n;

    insta::assert_snapshot!(default.length_lower_than(5));
    insta::assert_snapshot!(default.length_greater_than(10));
    insta::assert_snapshot!(default.range_lower_than("0"));
    insta::assert_snapshot!(default.range_greater_than("100"));
    insta::assert_snapshot!(default.credit_card_invalid("invalid format"));
    insta::assert_snapshot!(default.pattern_no_match("\\d+"));
    insta::assert_snapshot!(default.contains_missing("test"));
    insta::assert_snapshot!(default.url_invalid("invalid scheme"));
    insta::assert_snapshot!(default.prefix_missing("http://"));
    insta::assert_snapshot!(default.suffix_missing(".com"));
    insta::assert_snapshot!(default.phone_number_invalid());
    insta::assert_snapshot!(default.phone_number_invalid_with_error("wrong format"));
    insta::assert_snapshot!(default.ip_invalid("IPv4"));
    insta::assert_snapshot!(default.matches_field_mismatch("password"));
    insta::assert_snapshot!(default.email_invalid("missing @"));
    insta::assert_snapshot!(default.ascii_invalid());
    insta::assert_snapshot!(default.alphanumeric_invalid());
    insta::assert_snapshot!(default.required_not_set());
}

#[test]
fn test_custom_i18n_messages() {
    use garde::I18n as _;
    let custom = TestI18n;

    insta::assert_snapshot!(custom.length_lower_than(3));
    insta::assert_snapshot!(custom.length_greater_than(20));
    insta::assert_snapshot!(custom.range_lower_than("1"));
    insta::assert_snapshot!(custom.range_greater_than("99"));
    insta::assert_snapshot!(custom.credit_card_invalid("bad format"));
    insta::assert_snapshot!(custom.pattern_no_match("[a-z]+"));
    insta::assert_snapshot!(custom.contains_missing("hello"));
    insta::assert_snapshot!(custom.url_invalid("malformed"));
    insta::assert_snapshot!(custom.prefix_missing("www."));
    insta::assert_snapshot!(custom.suffix_missing(".org"));
    insta::assert_snapshot!(custom.phone_number_invalid());
    insta::assert_snapshot!(custom.phone_number_invalid_with_error("too short"));
    insta::assert_snapshot!(custom.ip_invalid("IPv6"));
    insta::assert_snapshot!(custom.matches_field_mismatch("confirm_password"));
    insta::assert_snapshot!(custom.email_invalid("no domain"));
    insta::assert_snapshot!(custom.ascii_invalid());
    insta::assert_snapshot!(custom.alphanumeric_invalid());
    insta::assert_snapshot!(custom.required_not_set());
}

#[test]
fn test_simple_validation_with_default_i18n() {
    util::check_fail!(
        &[User {
            name: "Jo".to_string(), // too short
            email: "invalid-email".to_string(),
            age: 15,     // too young
            phone: None, // required but not set
        }],
        &(),
    )
}

#[test]
fn test_simple_validation_with_custom_i18n() {
    util::check_fail!(
        &[User {
            name: "Jo".to_string(),
            email: "invalid-email".to_string(),
            age: 15,
            phone: None,
        }],
        &(),
    )
}

#[test]
fn test_nested_validation_with_default_i18n() {
    util::check_fail!(
        &[Profile {
            user: User {
                name: "A".to_string(),          // too short
                email: "bad-email".to_string(), // invalid
                age: 200,                       // too old
                phone: None,                    // required
            },
            address: Address {
                street: "St".to_string(),    // too short
                city: "X".to_string(),       // too short
                zip_code: "123".to_string(), // wrong format
            },
            bio: "Short".to_string(), // too short
        }],
        &(),
    )
}

#[test]
fn test_nested_validation_with_custom_i18n() {
    garde::with_i18n(TestI18n, || {
        util::check_fail!(
            &[Profile {
                user: User {
                    name: "A".to_string(),
                    email: "bad-email".to_string(),
                    age: 200,
                    phone: None,
                },
                address: Address {
                    street: "St".to_string(),
                    city: "X".to_string(),
                    zip_code: "123".to_string(),
                },
                bio: "Short".to_string(),
            }],
            &(),
        )
    });
}
