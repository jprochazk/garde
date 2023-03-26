use std::fmt::Display;
use std::str::FromStr;

use crate::error::Error;

#[cfg_attr(
    feature = "nightly-error-messages",
    rustc_on_unimplemented(
        message = "`{Self}` does not support phone number validation",
        label = "This type does not support phone number validation",
    )
)]
pub trait PhoneNumber {
    type Error: Display;

    fn try_parse_phone_number(&self) -> Result<(), Self::Error>;
}

pub fn apply<T: PhoneNumber>(v: &T, _: ()) -> Result<(), Error> {
    if let Err(e) = v.try_parse_phone_number() {
        return Err(Error::new(format!("not a valid phone number: {e}")));
    }
    Ok(())
}

fn check_str(v: &str) -> Result<(), phonenumber::ParseError> {
    let _ = phonenumber::PhoneNumber::from_str(v)?;
    Ok(())
}

impl PhoneNumber for String {
    type Error = phonenumber::ParseError;

    fn try_parse_phone_number(&self) -> Result<(), Self::Error> {
        check_str(self.as_str())
    }
}

impl<'a> PhoneNumber for &'a str {
    type Error = phonenumber::ParseError;

    fn try_parse_phone_number(&self) -> Result<(), Self::Error> {
        check_str(self)
    }
}

impl<'a> PhoneNumber for std::borrow::Cow<'a, str> {
    type Error = phonenumber::ParseError;

    fn try_parse_phone_number(&self) -> Result<(), Self::Error> {
        check_str(self.as_ref())
    }
}
