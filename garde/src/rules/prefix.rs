//! Prefix validation.
//!
//! ```rust
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(prefix("test_"))]
//!     v: String,
//! }
//! ```
//!
//! The entrypoint is the [`Prefix`] trait. Implementing this trait for a type allows that type to be used with the `#[garde(prefix)]` rule.
//!
//! This trait has a blanket implementation for all `T: AsRef<str>`.

use crate::error::Error;

pub fn apply<T: Prefix>(v: &T, (pat,): (&str,)) -> Result<(), Error> {
    if !v.validate_prefix(pat) {
        return Err(Error::new(format!("value does not begin with \"{pat}\"")));
    }
    Ok(())
}

#[cfg_attr(
    feature = "nightly-error-messages",
    rustc_on_unimplemented(
        message = "`{Self}` does not support prefix validation",
        label = "This type does not support prefix validation",
    )
)]
pub trait Prefix {
    fn validate_prefix(&self, pat: &str) -> bool;
}

impl<T: AsRef<str>> Prefix for T {
    fn validate_prefix(&self, pat: &str) -> bool {
        self.as_ref().starts_with(pat)
    }
}
