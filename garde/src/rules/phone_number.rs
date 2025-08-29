//! Phone number validation using the [`phonenumber`] crate.
//!
//! ```rust
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(phone_number)]
//!     v: String,
//! }
//! ```
//!
//! The entrypoint is the [`PhoneNumber`] trait. Implementing this trait for a type allows that type to be used with the `#[garde(phone_number)]` rule.
//!
//! This trait has a blanket implementation for all `T: garde::rules::AsStr`.

use std::str::FromStr;

use super::AsStr;
use crate::error::Error;
pub use crate::i18n::InvalidPhoneNumber;

pub fn apply<T: PhoneNumber>(v: &T, _: ()) -> Result<(), Error> {
    if let Err(reason) = v.validate_phone_number() {
        return Err(Error::new(i18n!(phone_number_invalid, reason)));
    }
    Ok(())
}

pub trait PhoneNumber {
    fn validate_phone_number(&self) -> Result<(), InvalidPhoneNumber>;
}

impl<T: AsStr> PhoneNumber for T {
    fn validate_phone_number(&self) -> Result<(), InvalidPhoneNumber> {
        let number = phonenumber::PhoneNumber::from_str(self.as_str())?;
        if number.is_valid() {
            Ok(())
        } else {
            Err(InvalidPhoneNumber::Invalid)
        }
    }
}

impl<T: PhoneNumber> PhoneNumber for Option<T> {
    fn validate_phone_number(&self) -> Result<(), InvalidPhoneNumber> {
        match self {
            Some(value) => value.validate_phone_number(),
            None => Ok(()),
        }
    }
}

impl From<phonenumber::ParseError> for InvalidPhoneNumber {
    fn from(e: phonenumber::ParseError) -> Self {
        match e {
            phonenumber::ParseError::NoNumber => InvalidPhoneNumber::NotANumber,
            phonenumber::ParseError::InvalidCountryCode => InvalidPhoneNumber::InvalidCountryCode,
            phonenumber::ParseError::TooShortAfterIdd => InvalidPhoneNumber::TooShortAfterIdd,
            phonenumber::ParseError::TooShortNsn => InvalidPhoneNumber::TooShortNsn,
            phonenumber::ParseError::TooLong => InvalidPhoneNumber::TooLong,
            phonenumber::ParseError::MalformedInteger(_) => InvalidPhoneNumber::MalformedInteger,
        }
    }
}
