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

use std::fmt::Display;
use std::str::FromStr;

use super::AsStr;
use crate::error::Error;

pub fn apply<T: PhoneNumber>(v: &T, _: ()) -> Result<(), Error> {
    if let Err(e) = v.validate_phone_number() {
        return Err(Error::new(
            "INVALID_PHONE_NUMBER",
            format!("not a valid phone number: {e}"),
        ));
    }
    Ok(())
}

pub trait PhoneNumber {
    type Error: Display;

    fn validate_phone_number(&self) -> Result<(), Self::Error>;
}

impl<T: AsStr> PhoneNumber for T {
    type Error = phonenumber::ParseError;

    fn validate_phone_number(&self) -> Result<(), Self::Error> {
        let _ = phonenumber::PhoneNumber::from_str(self.as_str())?;
        Ok(())
    }
}

impl<T: PhoneNumber> PhoneNumber for Option<T> {
    type Error = T::Error;

    fn validate_phone_number(&self) -> Result<(), Self::Error> {
        match self {
            Some(value) => value.validate_phone_number(),
            None => Ok(()),
        }
    }
}
