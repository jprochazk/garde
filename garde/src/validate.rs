use std::fmt::Debug;

use crate::error::Errors;

/// The core trait of this crate.
///
/// Validation checks all the conditions and returns all errors aggregated into
/// an `Errors` container.
pub trait Validate {
    /// A user-provided context.
    ///
    /// Custom validators receive a reference to this context.
    type Context;

    /// Validates `Self`, returning an `Err` with an aggregate of all errors if
    /// the validation failed.
    fn validate(&self, ctx: &Self::Context) -> Result<(), Errors>;
}

/// A struct which wraps a valid instance of some `T`.
///
/// The only way to create an instance of this struct is through the
/// [`Unvalidated`] type. This ensures that if you have a `Valid<T>`, it was
/// definitely validated at some point. This is commonly referred to as the
/// typestate pattern.
#[derive(Debug, Clone, Copy)]
pub struct Valid<T>(T);

impl<T: Validate> Valid<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T> std::ops::Deref for Valid<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A struct which wraps a potentially invalid instance of some `T`.
///
/// Use the `validate` method to turn this type into a `Valid<T>`.
#[derive(Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Unvalidated<T>(T);

impl<T: Validate> Unvalidated<T> {
    pub fn new(v: T) -> Self {
        Self(v)
    }

    pub fn validate(self, ctx: &<T as Validate>::Context) -> Result<Valid<T>, Errors> {
        self.0.validate(ctx)?;
        Ok(Valid(self.0))
    }
}

impl<T: Validate> From<T> for Unvalidated<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: Debug> Debug for Unvalidated<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}
