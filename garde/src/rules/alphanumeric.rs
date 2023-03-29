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
//! This trait has a blanket implementation for all `T: AsRef<str>`.

use crate::error::Error;

pub fn apply<T: Alphanumeric>(v: &T, _: ()) -> Result<(), Error> {
    if !v.validate_alphanumeric() {
        return Err(Error::new("not alphanumeric"));
    }
    Ok(())
}

#[cfg_attr(
    feature = "nightly-error-messages",
    rustc_on_unimplemented(
        message = "`{Self}` does not support alphanumeric validation",
        label = "This type does not support alphanumeric validation",
    )
)]
pub trait Alphanumeric {
    fn validate_alphanumeric(&self) -> bool;
}

impl<T: AsRef<str>> Alphanumeric for T {
    fn validate_alphanumeric(&self) -> bool {
        self.as_ref().chars().all(|c| c.is_alphanumeric())
    }
}
