//! Error message internationalization (i18n).
//!
//! This module provides a trait-based system for customizing validation error messages
//! in different languages or formats. By implementing the [`I18n`] trait, you can provide
//! custom error messages for validation rules supported by `garde`.
//!
//! # Customization
//!
//! By default, `garde` uses english error messages via the [`DefaultI18n`] implementation.
//!
//! To use custom error messages, implement the [`I18n`] trait, and validate inside a call to [`with_i18n`].
//!
//! ```rust,ignore
//! use std::borrow::Cow;
//! use std::fmt::Display;
//! use garde::{Validate, i18n::{I18n, with_i18n}};
//!
//! struct Czech;
//!
//! impl I18n for Czech {
//!     fn length_lower_than(&self, min: usize) -> Cow<'static, str> {
//!         format!("musí obsahovat alespoň {min} znaků").into()
//!     }
//!
//!     fn email_invalid(&self, error: &dyn Display) -> Cow<'static, str> {
//!         format!("email je neplatný: {error}").into()
//!     }
//!
//!     // etc.
//! }
//!
//! #[derive(Validate)]
//! struct User {
//!     #[garde(length(min = 3))]
//!     name: String,
//!     #[garde(email)]
//!     email: String,
//! }
//!
//! let user = User {
//!     name: "Jan Novák".to_string(),
//!     email: "invalid-email".to_string(),
//! };
//!
//! let result = with_i18n(Czech, || user.validate());
//! ```

use std::borrow::Cow;
use std::cell::Cell;
use std::fmt::Display;
use std::mem::transmute;
use std::ptr::NonNull;

pub use crate::rules::ip::IpKind;

/// Reasons an email value can fail to parse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
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
            InvalidEmail::Empty => f.write_str("value is empty"),
            InvalidEmail::MissingAt => f.write_str("value is missing `@`"),
            InvalidEmail::UserLengthExceeded => {
                f.write_str("user length exceeded maximum of 64 characters")
            }
            InvalidEmail::InvalidUser => f.write_str("user contains unexpected characters"),
            InvalidEmail::DomainLengthExceeded => {
                f.write_str("domain length exceeded maximum of 255 characters")
            }
            InvalidEmail::InvalidDomain => f.write_str("domain contains unexpected characters"),
        }
    }
}

/// Reasons a URL value can fail to parse.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum InvalidUrl {
    EmptyHost,
    IdnaError,
    InvalidPort,
    InvalidIpv4Address,
    InvalidIpv6Address,
    InvalidDomainCharacter,
    RelativeUrlWithoutBase,
    RelativeUrlWithCannotBeABaseBase,
    SetHostOnCannotBeABaseUrl,
    Overflow,
    Other,
}

impl Display for InvalidUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidUrl::EmptyHost => f.write_str("empty host"),
            InvalidUrl::IdnaError => f.write_str("invalid international domain name"),
            InvalidUrl::InvalidPort => f.write_str("invalid port number"),
            InvalidUrl::InvalidIpv4Address => f.write_str("invalid IPv4 address"),
            InvalidUrl::InvalidIpv6Address => f.write_str("invalid IPv6 address"),
            InvalidUrl::InvalidDomainCharacter => f.write_str("invalid domain character"),
            InvalidUrl::RelativeUrlWithoutBase => f.write_str("relative URL without a base"),
            InvalidUrl::RelativeUrlWithCannotBeABaseBase => {
                f.write_str("relative URL with a cannot-be-a-base base")
            }
            InvalidUrl::SetHostOnCannotBeABaseUrl => {
                f.write_str("a cannot-be-a-base URL doesn\u{2019}t have a host to set")
            }
            InvalidUrl::Overflow => f.write_str("URLs more than 4 GB are not supported"),
            InvalidUrl::Other => f.write_str("invalid url"),
        }
    }
}

/// Reasons a credit-card value can fail to validate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum InvalidCreditCard {
    InvalidFormat,
    InvalidLength,
    InvalidLuhn,
    UnknownType,
    Other,
}

impl Display for InvalidCreditCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidCreditCard::InvalidFormat => f.write_str("invalid format"),
            InvalidCreditCard::InvalidLength => f.write_str("invalid length"),
            InvalidCreditCard::InvalidLuhn => f.write_str("invalid luhn"),
            InvalidCreditCard::UnknownType => f.write_str("unknown type"),
            InvalidCreditCard::Other => f.write_str("invalid credit card"),
        }
    }
}

/// Reasons a phone-number value can fail to parse or validate.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum InvalidPhoneNumber {
    /// The number was parsed successfully, but failed validation.
    Invalid,

    NotANumber,
    InvalidCountryCode,
    TooShortAfterIdd,
    TooShortNsn,
    TooLong,
    MalformedInteger,
    Other,
}

impl Display for InvalidPhoneNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InvalidPhoneNumber::Invalid => f.write_str("not a valid phone number"),
            InvalidPhoneNumber::NotANumber => f.write_str("not a number"),
            InvalidPhoneNumber::InvalidCountryCode => f.write_str("invalid country code"),
            InvalidPhoneNumber::TooShortAfterIdd => {
                f.write_str("the number is too short after IDD")
            }
            InvalidPhoneNumber::TooShortNsn => {
                f.write_str("the number is too short after the country code")
            }
            InvalidPhoneNumber::TooLong => f.write_str("the number is too long"),
            InvalidPhoneNumber::MalformedInteger => {
                f.write_str("malformed integer part in phone number")
            }
            InvalidPhoneNumber::Other => f.write_str("invalid phone number"),
        }
    }
}

/// Trait for providing custom validation error messages.
///
/// See the [module-level documentation](crate::i18n) for usage.
pub trait I18n {
    /// Rule: `length`
    fn length_lower_than(&self, min: usize) -> Cow<'static, str>;

    /// Rule: `length`
    fn length_greater_than(&self, max: usize) -> Cow<'static, str>;

    /// Rule: `range`
    fn range_lower_than(&self, min: &dyn Display) -> Cow<'static, str>;

    /// Rule: `range`
    fn range_greater_than(&self, max: &dyn Display) -> Cow<'static, str>;

    /// Rule: `credit_card`
    fn credit_card_invalid(&self, reason: InvalidCreditCard) -> Cow<'static, str>;

    /// Rule: `pattern`
    fn pattern_no_match(&self, pattern: &dyn Display) -> Cow<'static, str>;

    /// Rule: `contains`
    fn contains_missing(&self, pattern: &dyn Display) -> Cow<'static, str>;

    /// Rule: `url`
    fn url_invalid(&self, reason: InvalidUrl) -> Cow<'static, str>;

    /// Rule: `prefix`
    fn prefix_missing(&self, pattern: &dyn Display) -> Cow<'static, str>;

    /// Rule: `suffix`
    fn suffix_missing(&self, pattern: &dyn Display) -> Cow<'static, str>;

    /// Rule: `phone_number`
    fn phone_number_invalid(&self, reason: InvalidPhoneNumber) -> Cow<'static, str>;

    /// Rule: `ip`
    fn ip_invalid(&self, kind: IpKind) -> Cow<'static, str>;

    /// Rule: `matches`
    fn matches_field_mismatch(&self, field: &dyn Display) -> Cow<'static, str>;

    /// Rule: `email`
    fn email_invalid(&self, reason: InvalidEmail) -> Cow<'static, str>;

    /// Rule: `ascii`
    fn ascii_invalid(&self) -> Cow<'static, str>;

    /// Rule: `alphanumeric`
    fn alphanumeric_invalid(&self) -> Cow<'static, str>;

    /// Rule: `required`
    fn required_not_set(&self) -> Cow<'static, str>;
}

impl<T: I18n + ?Sized> I18n for &T {
    #[inline]
    fn length_lower_than(&self, min: usize) -> Cow<'static, str> {
        (**self).length_lower_than(min)
    }
    #[inline]
    fn length_greater_than(&self, max: usize) -> Cow<'static, str> {
        (**self).length_greater_than(max)
    }
    #[inline]
    fn range_lower_than(&self, min: &dyn Display) -> Cow<'static, str> {
        (**self).range_lower_than(min)
    }
    #[inline]
    fn range_greater_than(&self, max: &dyn Display) -> Cow<'static, str> {
        (**self).range_greater_than(max)
    }
    #[inline]
    fn credit_card_invalid(&self, reason: InvalidCreditCard) -> Cow<'static, str> {
        (**self).credit_card_invalid(reason)
    }
    #[inline]
    fn pattern_no_match(&self, pattern: &dyn Display) -> Cow<'static, str> {
        (**self).pattern_no_match(pattern)
    }
    #[inline]
    fn contains_missing(&self, pattern: &dyn Display) -> Cow<'static, str> {
        (**self).contains_missing(pattern)
    }
    #[inline]
    fn url_invalid(&self, reason: InvalidUrl) -> Cow<'static, str> {
        (**self).url_invalid(reason)
    }
    #[inline]
    fn prefix_missing(&self, pattern: &dyn Display) -> Cow<'static, str> {
        (**self).prefix_missing(pattern)
    }
    #[inline]
    fn suffix_missing(&self, pattern: &dyn Display) -> Cow<'static, str> {
        (**self).suffix_missing(pattern)
    }
    #[inline]
    fn phone_number_invalid(&self, reason: InvalidPhoneNumber) -> Cow<'static, str> {
        (**self).phone_number_invalid(reason)
    }
    #[inline]
    fn ip_invalid(&self, kind: IpKind) -> Cow<'static, str> {
        (**self).ip_invalid(kind)
    }
    #[inline]
    fn matches_field_mismatch(&self, field: &dyn Display) -> Cow<'static, str> {
        (**self).matches_field_mismatch(field)
    }
    #[inline]
    fn email_invalid(&self, reason: InvalidEmail) -> Cow<'static, str> {
        (**self).email_invalid(reason)
    }
    #[inline]
    fn ascii_invalid(&self) -> Cow<'static, str> {
        (**self).ascii_invalid()
    }
    #[inline]
    fn alphanumeric_invalid(&self) -> Cow<'static, str> {
        (**self).alphanumeric_invalid()
    }
    #[inline]
    fn required_not_set(&self) -> Cow<'static, str> {
        (**self).required_not_set()
    }
}

/// Default implementation of [`I18n`] which provides english error messages.
pub struct DefaultI18n;

impl I18n for DefaultI18n {
    fn length_lower_than(&self, min: usize) -> Cow<'static, str> {
        format!("length is lower than {min}").into()
    }

    fn length_greater_than(&self, max: usize) -> Cow<'static, str> {
        format!("length is greater than {max}").into()
    }

    fn range_lower_than(&self, min: &dyn Display) -> Cow<'static, str> {
        format!("lower than {min}").into()
    }

    fn range_greater_than(&self, max: &dyn Display) -> Cow<'static, str> {
        format!("greater than {max}").into()
    }

    fn credit_card_invalid(&self, reason: InvalidCreditCard) -> Cow<'static, str> {
        format!("not a valid credit card number: {reason}").into()
    }

    fn pattern_no_match(&self, pattern: &dyn Display) -> Cow<'static, str> {
        format!("does not match pattern /{pattern}/").into()
    }

    fn contains_missing(&self, pattern: &dyn Display) -> Cow<'static, str> {
        format!("does not contain \"{pattern}\"").into()
    }

    fn url_invalid(&self, reason: InvalidUrl) -> Cow<'static, str> {
        format!("not a valid url: {reason}").into()
    }

    fn prefix_missing(&self, pattern: &dyn Display) -> Cow<'static, str> {
        format!("value does not begin with \"{pattern}\"").into()
    }

    fn suffix_missing(&self, pattern: &dyn Display) -> Cow<'static, str> {
        format!("does not end with \"{pattern}\"").into()
    }

    fn phone_number_invalid(&self, reason: InvalidPhoneNumber) -> Cow<'static, str> {
        match reason {
            InvalidPhoneNumber::Invalid => Cow::Borrowed("not a valid phone number"),
            _ => format!("not a valid phone number: {reason}").into(),
        }
    }

    fn ip_invalid(&self, kind: IpKind) -> Cow<'static, str> {
        format!("not a valid {kind} address").into()
    }

    fn matches_field_mismatch(&self, field: &dyn Display) -> Cow<'static, str> {
        format!("does not match {field} field").into()
    }

    fn email_invalid(&self, reason: InvalidEmail) -> Cow<'static, str> {
        format!("not a valid email: {reason}").into()
    }

    fn ascii_invalid(&self) -> Cow<'static, str> {
        Cow::Borrowed("not ascii")
    }

    fn alphanumeric_invalid(&self) -> Cow<'static, str> {
        Cow::Borrowed("not alphanumeric")
    }

    fn required_not_set(&self) -> Cow<'static, str> {
        Cow::Borrowed("not set")
    }
}

thread_local! {
    pub(crate) static I18N: Cell<Option<NonNull<dyn I18n + 'static>>> =
        const { Cell::new(None) };
}

/// Dispatch a method call to the currently-installed [`I18n`] handler.
macro_rules! i18n {
    ($handler:ident $(, $($args:expr),*)?) => {
        $crate::i18n::I18N.with(|slot| match slot.get() {
            None => <$crate::i18n::DefaultI18n as $crate::i18n::I18n>::$handler(
                &$crate::i18n::DefaultI18n,
                $($($args),*)?
            ),
            // SAFETY: The stack guard in `with_i18n` guarantees that the value
            // referenced in `I18N` is still live and safe to dereference at this point.
            Some(p) => unsafe { p.as_ref() }.$handler($($($args),*)?),
        })
    };
}

/// Execute a closure with a custom [`I18n`] handler.
///
/// This handler is only installed for the current thread.
pub fn with_i18n<'a, R>(mut handler: impl I18n + 'a, f: impl FnOnce() -> R) -> R {
    let handler: *mut (dyn I18n + 'a) = &raw mut handler;

    // SAFETY: The pointer is non-null for the duration of this stack frame.
    let ptr: NonNull<dyn I18n + 'static> = unsafe {
        NonNull::new_unchecked(
            transmute::<*mut (dyn I18n + 'a), *mut (dyn I18n + 'static)>(handler),
        )
    };

    // Stack guard which restores the previous value of `I18N` on drop.
    struct Reset {
        prev: Option<NonNull<dyn I18n + 'static>>,
    }
    impl Drop for Reset {
        fn drop(&mut self) {
            I18N.with(|c| c.set(self.prev));
        }
    }

    let _reset = Reset {
        prev: I18N.with(|c| c.replace(Some(ptr))),
    };
    f()
}
