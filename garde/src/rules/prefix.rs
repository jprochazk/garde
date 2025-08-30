//! Prefix validation.
//!
//! ```rust
//! const PRE: &str = "test_";
//!
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(prefix("test_"))]
//!     v: String,
//!     #[garde(prefix(PRE))]
//!     w: String,
//! }
//! ```
//!
//! The entrypoint is the [`Prefix`] trait. Implementing this trait for a type allows that type to be used with the `#[garde(prefix)]` rule.
//!
//! This trait has a blanket implementation for all `T: garde::rules::AsStr`.

use super::AsStr;
use crate::error::Error;

pub fn apply<T: Prefix>(v: &T, (pat,): (&str,)) -> Result<(), Error> {
    if !v.validate_prefix(pat) {
        return Err(Error::new(i18n!(prefix_missing, pat)));
    }
    Ok(())
}

pub trait Prefix {
    fn validate_prefix(&self, pat: &str) -> bool;
}

impl<T: AsStr> Prefix for T {
    fn validate_prefix(&self, pat: &str) -> bool {
        self.as_str().starts_with(pat)
    }
}

impl<T: Prefix> Prefix for Option<T> {
    fn validate_prefix(&self, pat: &str) -> bool {
        match self {
            Some(value) => value.validate_prefix(pat),
            None => true,
        }
    }
}
