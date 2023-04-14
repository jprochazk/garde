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
//! This trait has a blanket implementation for all `T: AsRef<str>`.

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

impl<T: AsRef<str>> Suffix for T {
    fn validate_suffix(&self, pat: &str) -> bool {
        self.as_ref().ends_with(pat)
    }
}
