//! Email validation.
//!
//! ```rust
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(email)]
//!     v: String,
//! }
//! ```
//!
//! The entrypoint is the [`Email`] trait. Implementing this trait for a type allows that type to be used with the `#[garde(email)]` rule.
//!
//! This trait has a blanket implementation for all `T: garde::rules::AsStr`.

use std::fmt::Display;
use std::str::FromStr;

use super::pattern::Matcher;
use super::AsStr;
use crate::error::Error;

macro_rules! init_regex {
    ($var:ident => $p:literal) => {
        #[cfg(not(all(feature = "js-sys", target_arch = "wasm32", target_os = "unknown")))]
        static $var: $crate::rules::pattern::regex::StaticPattern =
            $crate::rules::pattern::regex::init_pattern!($p);

        #[cfg(all(feature = "js-sys", target_arch = "wasm32", target_os = "unknown"))]
        static $var: $crate::rules::pattern::regex_js_sys::StaticPattern =
            $crate::rules::pattern::regex_js_sys::init_pattern!($p);
    };
}

pub fn apply<T: Email>(v: &T, _: ()) -> Result<(), Error> {
    if let Err(e) = v.validate_email() {
        return Err(Error::new(
            "INVALID_EMAIL",
            format!("not a valid email: {e}"),
        ));
    }
    Ok(())
}

pub trait Email {
    type Error: Display;

    fn validate_email(&self) -> Result<(), Self::Error>;
}

impl<T: AsStr> Email for T {
    type Error = InvalidEmail;

    fn validate_email(&self) -> Result<(), Self::Error> {
        parse_email(self.as_str())
    }
}

impl<T: Email> Email for Option<T> {
    type Error = T::Error;

    fn validate_email(&self) -> Result<(), Self::Error> {
        match self {
            Some(value) => value.validate_email(),
            None => Ok(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InvalidEmail {
    Empty,
    MissingAt,
    UserLengthExceeded,
    InvalidUser,
    DomainLengthExceeded,
    InvalidDomain,
}

impl Display for InvalidEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidEmail::Empty => write!(f, "value is empty"),
            InvalidEmail::MissingAt => write!(f, "value is missing `@`"),
            InvalidEmail::UserLengthExceeded => {
                write!(f, "user length exceeded maximum of 64 characters")
            }
            InvalidEmail::InvalidUser => write!(f, "user contains unexpected characters"),
            InvalidEmail::DomainLengthExceeded => {
                write!(f, "domain length exceeded maximum of 255 characters")
            }
            InvalidEmail::InvalidDomain => write!(f, "domain contains unexpected characters"),
        }
    }
}

pub fn parse_email(s: &str) -> Result<(), InvalidEmail> {
    if s.is_empty() {
        return Err(InvalidEmail::Empty);
    }

    let (user, domain) = s.split_once('@').ok_or(InvalidEmail::MissingAt)?;

    if user.len() > 64 {
        return Err(InvalidEmail::UserLengthExceeded);
    }

    init_regex! {
        USER_RE => r"(?i-u)^[a-z0-9.!#$%&'*+/=?^_`{|}~-]+\z"
    }

    if !USER_RE.is_match(user) {
        return Err(InvalidEmail::InvalidUser);
    }

    if domain.len() > 255 {
        return Err(InvalidEmail::DomainLengthExceeded);
    }

    if !is_valid_domain(domain) {
        #[cfg(not(feature = "email-idna"))]
        {
            return Err(InvalidEmail::InvalidDomain);
        }

        #[cfg(feature = "email-idna")]
        {
            match idna::domain_to_ascii(domain) {
                Ok(domain) => {
                    if !is_valid_domain(&domain) {
                        return Err(InvalidEmail::InvalidDomain);
                    }
                }
                Err(_) => return Err(InvalidEmail::InvalidDomain),
            }
        }
    }

    Ok(())
}

fn is_valid_domain(domain: &str) -> bool {
    init_regex! {
        DOMAIN_NAME_RE => r"(?i-u)^[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?(?:\.[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?)*$"
    };

    if DOMAIN_NAME_RE.is_match(domain) {
        return true;
    }

    if is_smtp_addr(domain) {
        return true;
    }

    false
}

fn is_smtp_addr(domain: &str) -> bool {
    let domain = match domain.strip_prefix('[') {
        Some(domain) => domain,
        None => return false,
    };
    let domain = match domain.strip_suffix(']') {
        Some(domain) => domain,
        None => return false,
    };
    std::net::IpAddr::from_str(domain).is_ok()
}

// Tests taken from `validator`, modified for this API
// https://github.com/Keats/validator/blob/09efa7e78e6fbc853a6a56af6904a00e2e6632b8/validator/src/validation/email.rs#L76
#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::*;

    #[test]
    fn test_parse_email() {
        // Test cases taken from Django
        // https://github.com/django/django/blob/master/tests/validators/tests.py#L48
        let tests = &[
            ("email@here.com", None),
            ("weirder-email@here.and.there.com", None),
            (r#"!def!xyz%abc@example.com"#, None),
            ("email@[127.0.0.1]", None),
            ("email@[2001:dB8::1]", None),
            ("email@[2001:dB8:0:0:0:0:0:1]", None),
            ("email@[::fffF:127.0.0.1]", None),
            ("example@valid-----hyphens.com", None),
            ("example@valid-with-hyphens.com", None),
            ("test@domain.with.idn.tld.उदाहरण.परीक्षा", None),
            (
                r#""test@test"@example.com"#,
                Some(InvalidEmail::InvalidUser),
            ),
            // max length for domain name labels is 63 characters per RFC 1034
            (
                "a@atm.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                None,
            ),
            (
                "a@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.atm",
                None,
            ),
            (
                "a@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.bbbbbbbbbb.atm",
                None,
            ),
            // 64 * a
            (
                "a@atm.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
                Some(InvalidEmail::InvalidDomain),
            ),
            ("", Some(InvalidEmail::Empty)),
            ("abc", Some(InvalidEmail::MissingAt)),
            ("abc@", Some(InvalidEmail::InvalidDomain)),
            ("abc@bar", None),
            ("a @x.cz", Some(InvalidEmail::InvalidUser)),
            ("abc@.com", Some(InvalidEmail::InvalidDomain)),
            (
                "something@@somewhere.com",
                Some(InvalidEmail::InvalidDomain),
            ),
            ("email@127.0.0.1", None),
            ("email@[127.0.0.256]", Some(InvalidEmail::InvalidDomain)),
            ("email@[2001:db8::12345]", Some(InvalidEmail::InvalidDomain)),
            (
                "email@[2001:db8:0:0:0:0:1]",
                Some(InvalidEmail::InvalidDomain),
            ),
            (
                "email@[::ffff:127.0.0.256]",
                Some(InvalidEmail::InvalidDomain),
            ),
            ("example@invalid-.com", Some(InvalidEmail::InvalidDomain)),
            ("example@-invalid.com", Some(InvalidEmail::InvalidDomain)),
            ("example@invalid.com-", Some(InvalidEmail::InvalidDomain)),
            ("example@inv-.alid-.com", Some(InvalidEmail::InvalidDomain)),
            ("example@inv-.-alid.com", Some(InvalidEmail::InvalidDomain)),
            (
                r#"test@example.com\n\n<script src="x.js">"#,
                Some(InvalidEmail::InvalidDomain),
            ),
            (r#""\\\011"@here.com"#, Some(InvalidEmail::InvalidUser)),
            (r#""\\\012"@here.com"#, Some(InvalidEmail::InvalidUser)),
            (
                "trailingdot@shouldfail.com.",
                Some(InvalidEmail::InvalidDomain),
            ),
            // Trailing newlines in username or domain not allowed
            ("a@b.com\n", Some(InvalidEmail::InvalidDomain)),
            ("a\n@b.com", Some(InvalidEmail::InvalidUser)),
            (
                r#""test@test"\n@example.com"#,
                Some(InvalidEmail::InvalidUser),
            ),
            ("a@[127.0.0.1]\n", Some(InvalidEmail::InvalidDomain)),
            // underscores are not allowed
            ("John.Doe@exam_ple.com", Some(InvalidEmail::InvalidDomain)),
        ];

        for (input, expected) in tests {
            // println!("{} - {}", input, expected);
            assert_eq!(
                &parse_email(input).err(),
                expected,
                "Email `{}` was not classified correctly",
                input
            );
        }
    }

    #[test]
    fn test_parse_email_cow() {
        let test: Cow<'static, str> = "email@here.com".into();
        assert!(parse_email(&test).is_ok());
        let test: Cow<'static, str> = String::from("email@here.com").into();
        assert!(parse_email(&test).is_ok());
        let test: Cow<'static, str> = "a@[127.0.0.1]\n".into();
        assert!(parse_email(&test).is_err());
        let test: Cow<'static, str> = String::from("a@[127.0.0.1]\n").into();
        assert!(parse_email(&test).is_err());
    }

    #[test]
    fn test_parse_email_rfc5321() {
        // 65 character local part
        let test = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa@mail.com";
        assert_eq!(
            parse_email(test).err(),
            Some(InvalidEmail::UserLengthExceeded)
        );
        // 256 character domain part
        let test = "a@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa.com";
        assert_eq!(
            parse_email(test).err(),
            Some(InvalidEmail::DomainLengthExceeded)
        );
    }
}
