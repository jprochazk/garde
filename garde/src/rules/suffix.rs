//! Suffix validation.
//!
//! ```rust
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(suffix("_test"))]
//!     v: String,
//! }
//! ```
//!
//! The entrypoint is the [`Suffix`] trait. Implementing this trait for a type allows that type to be used with the `#[garde(suffix)]` rule.
//!
//! This trait has a blanket implementation for all `T: garde::rules::AsStr`.

use super::AsStr;
use crate::error::Error;

pub fn apply<T: Suffix>(v: &T, (pat,): (&str,)) -> Result<(), Error> {
    if !v.validate_suffix(pat) {
        return Err(Error::new(format!("does not end with \"{pat}\"")));
    }
    Ok(())
}

pub trait Suffix {
    fn validate_suffix(&self, pat: &str) -> bool;
}

impl<T: AsStr> Suffix for T {
    fn validate_suffix(&self, pat: &str) -> bool {
        self.as_str().ends_with(pat)
    }
}

impl<T: Suffix> Suffix for Option<T> {
    fn validate_suffix(&self, pat: &str) -> bool {
        match self {
            Some(value) => value.validate_suffix(pat),
            None => true,
        }
    }
}
