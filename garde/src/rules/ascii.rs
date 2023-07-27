//! ASCII validation.
//!
//! ```rust
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(ascii)]
//!     v: String,
//! }
//! ```
//!
//! The entrypoint is the [`Ascii`] trait. Implementing this trait for a type allows that type to be used with the `#[garde(ascii)]` rule.
//!
//! This trait has a blanket implementation for all `T: garde::rules::AsStr`.

use super::AsStr;
use crate::error::Error;

pub fn apply<T: Ascii>(v: &T, _: ()) -> Result<(), Error> {
    if !v.validate_ascii() {
        return Err(Error::new("not ascii"));
    }
    Ok(())
}

pub trait Ascii {
    fn validate_ascii(&self) -> bool;
}

impl<T: AsStr> Ascii for T {
    fn validate_ascii(&self) -> bool {
        self.as_str().is_ascii()
    }
}

impl<T: Ascii> Ascii for Option<T> {
    fn validate_ascii(&self) -> bool {
        match self {
            Some(value) => value.validate_ascii(),
            None => true,
        }
    }
}
