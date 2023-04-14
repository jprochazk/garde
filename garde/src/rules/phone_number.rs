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
//! This trait has a blanket implementation for all `T: AsRef<str>`.

use std::fmt::Display;
use std::str::FromStr;

use crate::error::Error;

pub fn apply<T: PhoneNumber>(v: &T, _: ()) -> Result<(), Error> {
    if let Err(e) = v.validate_phone_number() {
        return Err(Error::new(format!("not a valid phone number: {e}")));
    }
    Ok(())
}

pub trait PhoneNumber {
    type Error: Display;

    fn validate_phone_number(&self) -> Result<(), Self::Error>;
}

impl<T: AsRef<str>> PhoneNumber for T {
    type Error = phonenumber::ParseError;

    fn validate_phone_number(&self) -> Result<(), Self::Error> {
        let _ = phonenumber::PhoneNumber::from_str(self.as_ref())?;
        Ok(())
    }
}
