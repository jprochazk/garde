//! ## Validation rules

pub mod alphanumeric;
pub mod ascii;
pub mod contains;
#[cfg(feature = "credit-card")]
pub mod credit_card;
#[cfg(feature = "email")]
pub mod email;
pub mod inner;
pub mod ip;
pub mod length;
pub mod matches;
pub mod pattern;
#[cfg(feature = "phone-number")]
pub mod phone_number;
pub mod prefix;
pub mod range;
pub mod required;
pub mod suffix;
#[cfg(feature = "url")]
pub mod url;

pub trait AsStr {
    fn as_str(&self) -> &str;
}

impl AsStr for &str {
    fn as_str(&self) -> &str {
        self
    }
}

impl AsStr for String {
    fn as_str(&self) -> &str {
        String::as_str(self)
    }
}

impl AsStr for std::borrow::Cow<'_, str> {
    fn as_str(&self) -> &str {
        std::borrow::Cow::as_ref(self)
    }
}
