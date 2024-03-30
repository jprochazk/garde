//! Alphanumeric validation.
//!
//! ```rust
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(alphanumeric)]
//!     v: String,
//! }
//! ```
//!
//! The entrypoint is the [`Alphanumeric`] trait. Implementing this trait for a type allows that type to be used with the `#[garde(alphanumeric)]` rule.
//!
//! This trait has a blanket implementation for all `T: garde::rules::AsStr`.

use super::AsStr;
use crate::error::Error;

pub fn apply<T: Alphanumeric>(v: &T, _: ()) -> Result<(), Error> {
    if !v.validate_alphanumeric() {
        return Err(Error::new("NOT_ALPHANUMERIC", "not alphanumeric"));
    }
    Ok(())
}

pub trait Alphanumeric {
    fn validate_alphanumeric(&self) -> bool;
}

impl<T: AsStr> Alphanumeric for T {
    fn validate_alphanumeric(&self) -> bool {
        self.as_str().chars().all(|c| c.is_alphanumeric())
    }
}

impl<T: Alphanumeric> Alphanumeric for Option<T> {
    fn validate_alphanumeric(&self) -> bool {
        match self {
            Some(value) => value.validate_alphanumeric(),
            None => true,
        }
    }
}
