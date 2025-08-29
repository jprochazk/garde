use std::borrow::Cow;
use std::fmt::Display;

use garde::i18n::{InvalidCreditCard, InvalidEmail, InvalidPhoneNumber, InvalidUrl, IpKind};

use super::util;

struct TestI18n;

impl garde::I18n for TestI18n {
    fn length_lower_than(&self, min: usize) -> Cow<'static, str> {
        format!("custom: too short, need at least {min}").into()
    }

    fn length_greater_than(&self, max: usize) -> Cow<'static, str> {
        format!("custom: too long, maximum is {max}").into()
    }

    fn range_lower_than(&self, min: &dyn Display) -> Cow<'static, str> {
        format!("custom: value too small, minimum is {min}").into()
    }

    fn range_greater_than(&self, max: &dyn Display) -> Cow<'static, str> {
        format!("custom: value too large, maximum is {max}").into()
    }

    fn credit_card_invalid(&self, reason: InvalidCreditCard) -> Cow<'static, str> {
        format!("custom: invalid credit card - {reason:?}").into()
    }

    fn pattern_no_match(&self, pattern: &dyn Display) -> Cow<'static, str> {
        format!("custom: doesn't match /{pattern}/").into()
    }

    fn contains_missing(&self, pattern: &dyn Display) -> Cow<'static, str> {
        format!("custom: missing required text '{pattern}'").into()
    }

    fn url_invalid(&self, reason: InvalidUrl) -> Cow<'static, str> {
        format!("custom: bad URL - {reason:?}").into()
    }

    fn prefix_missing(&self, pattern: &dyn Display) -> Cow<'static, str> {
        format!("custom: must start with '{pattern}'").into()
    }

    fn suffix_missing(&self, pattern: &dyn Display) -> Cow<'static, str> {
        format!("custom: must end with '{pattern}'").into()
    }

    fn phone_number_invalid(&self, reason: InvalidPhoneNumber) -> Cow<'static, str> {
        format!("custom: phone number error - {reason:?}").into()
    }

    fn ip_invalid(&self, kind: IpKind) -> Cow<'static, str> {
        format!("custom: not a valid {kind} IP").into()
    }

    fn matches_field_mismatch(&self, field: &dyn Display) -> Cow<'static, str> {
        format!("custom: must match {field}").into()
    }

    fn email_invalid(&self, reason: InvalidEmail) -> Cow<'static, str> {
        format!("custom: bad email - {reason:?}").into()
    }

    fn ascii_invalid(&self) -> Cow<'static, str> {
        "custom: contains non-ASCII characters".into()
    }

    fn alphanumeric_invalid(&self) -> Cow<'static, str> {
        "custom: only letters and numbers allowed".into()
    }

    fn required_not_set(&self) -> Cow<'static, str> {
        "custom: this field is required".into()
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
    insta::assert_snapshot!(default.range_lower_than(&0));
    insta::assert_snapshot!(default.range_greater_than(&100));
    insta::assert_snapshot!(default.credit_card_invalid(InvalidCreditCard::InvalidLuhn));
    insta::assert_snapshot!(default.pattern_no_match(&"\\d+"));
    insta::assert_snapshot!(default.contains_missing(&"test"));
    insta::assert_snapshot!(default.url_invalid(InvalidUrl::EmptyHost));
    insta::assert_snapshot!(default.prefix_missing(&"http://"));
    insta::assert_snapshot!(default.suffix_missing(&".com"));
    insta::assert_snapshot!(default.phone_number_invalid(InvalidPhoneNumber::Invalid));
    insta::assert_snapshot!(default.phone_number_invalid(InvalidPhoneNumber::TooLong));
    insta::assert_snapshot!(default.ip_invalid(IpKind::V4));
    insta::assert_snapshot!(default.matches_field_mismatch(&"password"));
    insta::assert_snapshot!(default.email_invalid(InvalidEmail::MissingAt));
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
    insta::assert_snapshot!(custom.range_lower_than(&1));
    insta::assert_snapshot!(custom.range_greater_than(&99));
    insta::assert_snapshot!(custom.credit_card_invalid(InvalidCreditCard::InvalidLength));
    insta::assert_snapshot!(custom.pattern_no_match(&"[a-z]+"));
    insta::assert_snapshot!(custom.contains_missing(&"hello"));
    insta::assert_snapshot!(custom.url_invalid(InvalidUrl::Overflow));
    insta::assert_snapshot!(custom.prefix_missing(&"www."));
    insta::assert_snapshot!(custom.suffix_missing(&".org"));
    insta::assert_snapshot!(custom.phone_number_invalid(InvalidPhoneNumber::Invalid));
    insta::assert_snapshot!(custom.phone_number_invalid(InvalidPhoneNumber::TooShortNsn));
    insta::assert_snapshot!(custom.ip_invalid(IpKind::V6));
    insta::assert_snapshot!(custom.matches_field_mismatch(&"confirm_password"));
    insta::assert_snapshot!(custom.email_invalid(InvalidEmail::InvalidDomain));
    insta::assert_snapshot!(custom.ascii_invalid());
    insta::assert_snapshot!(custom.alphanumeric_invalid());
    insta::assert_snapshot!(custom.required_not_set());
}

#[test]
fn test_simple_validation_with_default_i18n() {
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
fn test_simple_validation_with_custom_i18n() {
    garde::with_i18n(TestI18n, || {
        util::check_fail!(
            &[User {
                name: "Jo".to_string(),
                email: "invalid-email".to_string(),
                age: 15,
                phone: None,
            }],
            &(),
        )
    });
}

#[test]
fn test_nested_validation_with_default_i18n() {
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

#[derive(Debug, garde::Validate)]
struct TooShort {
    #[garde(length(min = 5))]
    s: String,
}

fn first_error_message<T: garde::Validate<Context = ()>>(v: &T) -> String {
    let report = v.validate().unwrap_err();
    let (_path, err) = report.iter().next().expect("expected at least one error");
    err.to_string()
}

/// `length_lower_than` is overridden; everything else forwards to `DefaultI18n`.
struct LengthOnly(&'static str);

impl garde::I18n for LengthOnly {
    fn length_lower_than(&self, min: usize) -> Cow<'static, str> {
        format!("{}:{min}", self.0).into()
    }
    fn length_greater_than(&self, m: usize) -> Cow<'static, str> {
        garde::i18n::DefaultI18n.length_greater_than(m)
    }
    fn range_lower_than(&self, x: &dyn Display) -> Cow<'static, str> {
        garde::i18n::DefaultI18n.range_lower_than(x)
    }
    fn range_greater_than(&self, x: &dyn Display) -> Cow<'static, str> {
        garde::i18n::DefaultI18n.range_greater_than(x)
    }
    fn credit_card_invalid(&self, r: InvalidCreditCard) -> Cow<'static, str> {
        garde::i18n::DefaultI18n.credit_card_invalid(r)
    }
    fn pattern_no_match(&self, x: &dyn Display) -> Cow<'static, str> {
        garde::i18n::DefaultI18n.pattern_no_match(x)
    }
    fn contains_missing(&self, x: &dyn Display) -> Cow<'static, str> {
        garde::i18n::DefaultI18n.contains_missing(x)
    }
    fn url_invalid(&self, r: InvalidUrl) -> Cow<'static, str> {
        garde::i18n::DefaultI18n.url_invalid(r)
    }
    fn prefix_missing(&self, x: &dyn Display) -> Cow<'static, str> {
        garde::i18n::DefaultI18n.prefix_missing(x)
    }
    fn suffix_missing(&self, x: &dyn Display) -> Cow<'static, str> {
        garde::i18n::DefaultI18n.suffix_missing(x)
    }
    fn phone_number_invalid(&self, r: InvalidPhoneNumber) -> Cow<'static, str> {
        garde::i18n::DefaultI18n.phone_number_invalid(r)
    }
    fn ip_invalid(&self, k: IpKind) -> Cow<'static, str> {
        garde::i18n::DefaultI18n.ip_invalid(k)
    }
    fn matches_field_mismatch(&self, x: &dyn Display) -> Cow<'static, str> {
        garde::i18n::DefaultI18n.matches_field_mismatch(x)
    }
    fn email_invalid(&self, r: InvalidEmail) -> Cow<'static, str> {
        garde::i18n::DefaultI18n.email_invalid(r)
    }
    fn ascii_invalid(&self) -> Cow<'static, str> {
        garde::i18n::DefaultI18n.ascii_invalid()
    }
    fn alphanumeric_invalid(&self) -> Cow<'static, str> {
        garde::i18n::DefaultI18n.alphanumeric_invalid()
    }
    fn required_not_set(&self) -> Cow<'static, str> {
        garde::i18n::DefaultI18n.required_not_set()
    }
}

#[test]
fn test_nesting_restores_outer_handler() {
    let v = TooShort { s: "ab".into() };

    assert_eq!(first_error_message(&v), "length is lower than 5");

    garde::with_i18n(LengthOnly("OUTER"), || {
        assert_eq!(first_error_message(&v), "OUTER:5");

        garde::with_i18n(LengthOnly("INNER"), || {
            assert_eq!(first_error_message(&v), "INNER:5");
        });

        // Outer must be restored after inner exits.
        assert_eq!(first_error_message(&v), "OUTER:5");
    });

    // And cleared after the outermost scope.
    assert_eq!(first_error_message(&v), "length is lower than 5");
}

#[test]
fn test_panic_safety() {
    let v = TooShort { s: "ab".into() };

    let _ = std::panic::catch_unwind(|| {
        garde::with_i18n(LengthOnly("OUTER"), || panic!("boom"));
    });

    // If the slot weren't cleared on unwind, this would say "OUTER:5".
    assert_eq!(first_error_message(&v), "length is lower than 5");
}

#[test]
fn test_borrowed_handler() {
    // Non-'static handler via the `impl<T: I18n + ?Sized> I18n for &T` blanket.
    let label = String::from("cs-CZ");
    // Leak-free borrow: the &str lives in `label`, which outlives `with_i18n`.
    let prefix: &str = label.as_str();

    struct Borrowed<'a>(&'a str);
    impl<'a> garde::I18n for Borrowed<'a> {
        fn length_lower_than(&self, min: usize) -> Cow<'static, str> {
            format!("[{}] {min}", self.0).into()
        }
        fn length_greater_than(&self, m: usize) -> Cow<'static, str> {
            garde::i18n::DefaultI18n.length_greater_than(m)
        }
        fn range_lower_than(&self, x: &dyn Display) -> Cow<'static, str> {
            garde::i18n::DefaultI18n.range_lower_than(x)
        }
        fn range_greater_than(&self, x: &dyn Display) -> Cow<'static, str> {
            garde::i18n::DefaultI18n.range_greater_than(x)
        }
        fn credit_card_invalid(&self, r: InvalidCreditCard) -> Cow<'static, str> {
            garde::i18n::DefaultI18n.credit_card_invalid(r)
        }
        fn pattern_no_match(&self, x: &dyn Display) -> Cow<'static, str> {
            garde::i18n::DefaultI18n.pattern_no_match(x)
        }
        fn contains_missing(&self, x: &dyn Display) -> Cow<'static, str> {
            garde::i18n::DefaultI18n.contains_missing(x)
        }
        fn url_invalid(&self, r: InvalidUrl) -> Cow<'static, str> {
            garde::i18n::DefaultI18n.url_invalid(r)
        }
        fn prefix_missing(&self, x: &dyn Display) -> Cow<'static, str> {
            garde::i18n::DefaultI18n.prefix_missing(x)
        }
        fn suffix_missing(&self, x: &dyn Display) -> Cow<'static, str> {
            garde::i18n::DefaultI18n.suffix_missing(x)
        }
        fn phone_number_invalid(&self, r: InvalidPhoneNumber) -> Cow<'static, str> {
            garde::i18n::DefaultI18n.phone_number_invalid(r)
        }
        fn ip_invalid(&self, k: IpKind) -> Cow<'static, str> {
            garde::i18n::DefaultI18n.ip_invalid(k)
        }
        fn matches_field_mismatch(&self, x: &dyn Display) -> Cow<'static, str> {
            garde::i18n::DefaultI18n.matches_field_mismatch(x)
        }
        fn email_invalid(&self, r: InvalidEmail) -> Cow<'static, str> {
            garde::i18n::DefaultI18n.email_invalid(r)
        }
        fn ascii_invalid(&self) -> Cow<'static, str> {
            garde::i18n::DefaultI18n.ascii_invalid()
        }
        fn alphanumeric_invalid(&self) -> Cow<'static, str> {
            garde::i18n::DefaultI18n.alphanumeric_invalid()
        }
        fn required_not_set(&self) -> Cow<'static, str> {
            garde::i18n::DefaultI18n.required_not_set()
        }
    }

    let b = Borrowed(prefix);
    let v = TooShort { s: "ab".into() };
    garde::with_i18n(&b, || {
        assert_eq!(first_error_message(&v), "[cs-CZ] 5");
    });
}
