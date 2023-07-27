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

use crate::error::ListErrorBuilder;
use crate::Errors;

pub fn apply<T, U, C, F>(field: &T, ctx: &C, f: F) -> Errors
where
    T: Inner<U>,
    F: Fn(&U, &C) -> Errors,
{
    field.validate_inner(ctx, f)
}

pub trait Inner<T> {
    type ErrorBuilder;

    fn validate_inner<C, F>(&self, ctx: &C, f: F) -> Errors
    where
        F: Fn(&T, &C) -> Errors;
}

impl<T> Inner<T> for Vec<T> {
    type ErrorBuilder = ListErrorBuilder;

    fn validate_inner<C, F>(&self, ctx: &C, f: F) -> Errors
    where
        F: Fn(&T, &C) -> Errors,
    {
        self.as_slice().validate_inner(ctx, f)
    }
}

impl<const N: usize, T> Inner<T> for [T; N] {
    type ErrorBuilder = ListErrorBuilder;

    fn validate_inner<C, F>(&self, ctx: &C, f: F) -> Errors
    where
        F: Fn(&T, &C) -> Errors,
    {
        self.as_slice().validate_inner(ctx, f)
    }
}

impl<'a, T> Inner<T> for &'a [T] {
    type ErrorBuilder = ListErrorBuilder;

    fn validate_inner<C, F>(&self, ctx: &C, f: F) -> Errors
    where
        F: Fn(&T, &C) -> Errors,
    {
        Errors::list(|b| {
            for item in self.iter() {
                b.push(f(item, ctx));
            }
        })
    }
}
