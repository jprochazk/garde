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

use super::AsStr;
use crate::error::Error;
pub use crate::i18n::InvalidCreditCard;

pub fn apply<T: CreditCard>(v: &T, _: ()) -> Result<(), Error> {
    if let Err(reason) = v.validate_credit_card() {
        return Err(Error::new(i18n!(credit_card_invalid, reason)));
    }
    Ok(())
}

pub trait CreditCard {
    fn validate_credit_card(&self) -> Result<(), InvalidCreditCard>;
}

impl<T: AsStr> CreditCard for T {
    fn validate_credit_card(&self) -> Result<(), InvalidCreditCard> {
        card_validate::Validate::from(self.as_str())
            .map(|_| ())
            .map_err(InvalidCreditCard::from)
    }
}

impl<T: CreditCard> CreditCard for Option<T> {
    fn validate_credit_card(&self) -> Result<(), InvalidCreditCard> {
        match self {
            Some(value) => value.validate_credit_card(),
            None => Ok(()),
        }
    }
}

impl From<card_validate::ValidateError> for InvalidCreditCard {
    fn from(e: card_validate::ValidateError) -> Self {
        match e {
            card_validate::ValidateError::InvalidFormat => InvalidCreditCard::InvalidFormat,
            card_validate::ValidateError::InvalidLength => InvalidCreditCard::InvalidLength,
            card_validate::ValidateError::InvalidLuhn => InvalidCreditCard::InvalidLuhn,
            card_validate::ValidateError::UnknownType => InvalidCreditCard::UnknownType,
            _ => InvalidCreditCard::Other,
        }
    }
}
