//! Inner type validation.
//!
//! ```rust
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(inner(length(min=1)))]
//!     v: Vec<String>,
//! }
//! ```
//!
//! The entrypoint is the [`Inner`] trait. Implementing this trait for a type allows that type to be used with the `#[garde(inner(..))]` rule.

use crate::error::{Path, Report};

pub fn apply<T, U, C, F>(field: &T, ctx: &C, current_path: Path, report: &mut Report, f: F)
where
    T: Inner<U>,
    F: Fn(&U, &C, Path, &mut Report),
{
    field.validate_inner(ctx, current_path, report, f)
}

pub trait Inner<T> {
    fn validate_inner<C, F>(&self, ctx: &C, current_path: Path, report: &mut Report, f: F)
    where
        F: Fn(&T, &C, Path, &mut Report);
}

impl<T> Inner<T> for Vec<T> {
    fn validate_inner<C, F>(&self, ctx: &C, current_path: Path, report: &mut Report, f: F)
    where
        F: Fn(&T, &C, Path, &mut Report),
    {
        self.as_slice().validate_inner(ctx, current_path, report, f)
    }
}

impl<const N: usize, T> Inner<T> for [T; N] {
    fn validate_inner<C, F>(&self, ctx: &C, current_path: Path, report: &mut Report, f: F)
    where
        F: Fn(&T, &C, Path, &mut Report),
    {
        self.as_slice().validate_inner(ctx, current_path, report, f)
    }
}

impl<'a, T> Inner<T> for &'a [T] {
    fn validate_inner<C, F>(&self, ctx: &C, current_path: Path, report: &mut Report, f: F)
    where
        F: Fn(&T, &C, Path, &mut Report),
    {
        for (index, item) in self.iter().enumerate() {
            f(item, ctx, current_path.join(index), report);
        }
    }
}
