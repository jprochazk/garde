//! Substring validation.
//!
//! ```rust
//! const STR: &str = "test";
//!
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(contains("test"))]
//!     v: String,
//!     #[garde(contains(STR))]
//!     w: String,
//! }
//! ```
//!
//! The entrypoint is the [`Contains`] trait. Implementing this trait for a type allows that type to be used with the `#[garde(contains)]` rule.
//!
//! This trait has a blanket implementation for all `T: garde::rules::AsStr`.

use super::AsStr;
use crate::error::Error;

pub fn apply<T: Contains>(v: &T, (pat,): (&str,)) -> Result<(), Error> {
    if !v.validate_contains(pat) {
        return Err(Error::new(format!("does not contain \"{pat}\"")));
    }
    Ok(())
}

pub trait Contains {
    fn validate_contains(&self, pat: &str) -> bool;
}

impl<T: AsStr> Contains for T {
    fn validate_contains(&self, pat: &str) -> bool {
        self.as_str().contains(pat)
    }
}

impl<T: Contains> Contains for Option<T> {
    fn validate_contains(&self, pat: &str) -> bool {
        match self {
            Some(value) => value.validate_contains(pat),
            None => true,
        }
    }
}
