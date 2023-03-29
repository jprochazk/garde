//! Substring validation.
//!
//! ```rust
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(contains("test"))]
//!     v: String,
//! }
//! ```
//!
//! The entrypoint is the [`Contains`] trait. Implementing this trait for a type allows that type to be used with the `#[garde(contains)]` rule.
//!
//! This trait has a blanket implementation for all `T: AsRef<str>`.

use crate::error::Error;

pub fn apply<T: Contains>(v: &T, (pat,): (&str,)) -> Result<(), Error> {
    if !v.validate_contains(pat) {
        return Err(Error::new(format!("does not contain \"{pat}\"")));
    }
    Ok(())
}

#[cfg_attr(
    feature = "nightly-error-messages",
    rustc_on_unimplemented(
        message = "`{Self}` does not support substring validation",
        label = "This type does not support substring validation",
    )
)]
pub trait Contains {
    fn validate_contains(&self, pat: &str) -> bool;
}

impl<T: AsRef<str>> Contains for T {
    fn validate_contains(&self, pat: &str) -> bool {
        self.as_ref().contains(pat)
    }
}
