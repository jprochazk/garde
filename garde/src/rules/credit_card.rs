use std::fmt::Display;

use crate::error::Error;

pub fn apply<T: CreditCard>(field_name: &str, v: &T) -> Result<(), Error> {
    if let Err(e) = v.try_parse_credit_card() {
        return Err(Error::new(
            format!("`{field_name}` is not a valid credit card number: {e}").into(),
        ));
    }
    Ok(())
}

#[cfg_attr(
    feature = "nightly-error-messages",
    rustc_on_unimplemented(
        message = "`{Self}` does not support credit card validation",
        label = "This type does not support credit card validation",
    )
)]
pub trait CreditCard {
    type Error: Display;

    fn try_parse_credit_card(&self) -> Result<(), Self::Error>;
}

fn check_str(s: &str) -> Result<(), InvalidCard> {
    let _ = card_validate::Validate::from(s)?;
    Ok(())
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

impl CreditCard for String {
    type Error = InvalidCard;

    fn try_parse_credit_card(&self) -> Result<(), Self::Error> {
        check_str(self.as_str())
    }
}
impl<'a> CreditCard for &'a str {
    type Error = InvalidCard;

    fn try_parse_credit_card(&self) -> Result<(), Self::Error> {
        check_str(self)
    }
}
impl<'a> CreditCard for std::borrow::Cow<'a, str> {
    type Error = InvalidCard;

    fn try_parse_credit_card(&self) -> Result<(), Self::Error> {
        check_str(self.as_ref())
    }
}
