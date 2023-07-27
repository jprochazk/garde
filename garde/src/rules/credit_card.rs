//! Credit card validation using the [`card_validate`] crate.
//!
//! ```rust
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(credit_card)]
//!     v: String,
//! }
//! ```
//!
//! The entrypoint is the [`CreditCard`] trait. Implementing this trait for a type allows that type to be used with the `#[garde(credit_card)]` rule.
//!
//! This trait has a blanket implementation for all `T: garde::rules::AsStr`.

use std::fmt::Display;

use super::AsStr;
use crate::error::Error;

pub fn apply<T: CreditCard>(v: &T, _: ()) -> Result<(), Error> {
    if let Err(e) = v.validate_credit_card() {
        return Err(Error::new(format!("not a valid credit card number: {e}")));
    }
    Ok(())
}

pub trait CreditCard {
    type Error: Display;

    fn validate_credit_card(&self) -> Result<(), Self::Error>;
}

impl<T: AsStr> CreditCard for T {
    type Error = InvalidCard;

    fn validate_credit_card(&self) -> Result<(), Self::Error> {
        let _ = card_validate::Validate::from(self.as_str())?;
        Ok(())
    }
}

impl<T: CreditCard> CreditCard for Option<T> {
    type Error = T::Error;

    fn validate_credit_card(&self) -> Result<(), Self::Error> {
        match self {
            Some(value) => value.validate_credit_card(),
            None => Ok(()),
        }
    }
}

pub struct InvalidCard(card_validate::ValidateError);
impl Display for InvalidCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            card_validate::ValidateError::InvalidFormat => write!(f, "invalid format"),
            card_validate::ValidateError::InvalidLength => write!(f, "invalid length"),
            card_validate::ValidateError::InvalidLuhn => write!(f, "invalid luhn"),
            card_validate::ValidateError::UnknownType => write!(f, "unknown type"),
            _ => write!(f, "unknown error"),
        }
    }
}

impl From<card_validate::ValidateError> for InvalidCard {
    fn from(value: card_validate::ValidateError) -> Self {
        Self(value)
    }
}
