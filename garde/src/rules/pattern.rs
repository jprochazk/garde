//! Pattern validation using the [`regex`] crate.
//!
//! ```rust
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(pattern(r"[a-zA-Z0-9][a-zA-Z0-9_]+"))]
//!     v: String,
//! }
//! ```
//!
//! The pattern argument can also be provided directly as an expression that references a value of type `Regex`.
//!
//! ```rust
//! use once_cell::sync::Lazy;
//! use regex::Regex;
//!
//! static LAZY_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"[a-zA-Z0-9][a-zA-Z0-9_]+").unwrap());
//!
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(pattern(LAZY_RE))]
//!     v: String,
//! }
//! ```
//!
//! The entrypoint is the [`Pattern`] trait. Implementing this trait for a type allows that type to be used with the `#[garde(pattern(...))]` rule.
//!
//! This trait has a blanket implementation for all `T: garde::rules::AsStr`.

use crate::error::Error;

pub fn apply<T: Pattern>(v: &T, (pat,): (&regex::Regex,)) -> Result<(), Error> {
    if !v.validate_pattern(pat) {
        return Err(Error::new(format!("does not match pattern /{pat}/")));
    }
    Ok(())
}

pub trait Pattern {
    fn validate_pattern(&self, pat: &Regex) -> bool;
}

#[doc(hidden)]
pub type StaticPattern = once_cell::sync::Lazy<Regex>;

#[doc(hidden)]
pub use regex::Regex;

#[doc(hidden)]
#[macro_export]
macro_rules! __init_pattern {
    ($pat:literal) => {
        $crate::rules::pattern::StaticPattern::new(|| {
            $crate::rules::pattern::Regex::new($pat).unwrap()
        })
    };
}
use super::AsStr;
#[doc(hidden)]
pub use crate::__init_pattern as init_pattern;

impl<T: AsStr> Pattern for T {
    fn validate_pattern(&self, pat: &Regex) -> bool {
        pat.is_match(self.as_str())
    }
}

impl<T: Pattern> Pattern for Option<T> {
    fn validate_pattern(&self, pat: &Regex) -> bool {
        match self {
            Some(value) => value.validate_pattern(pat),
            None => true,
        }
    }
}
