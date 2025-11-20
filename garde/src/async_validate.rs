//! ## Core validation traits and types

use std::fmt::Debug;
use std::future::Future;

use crate::error::Path;
use crate::{Report, Valid};

pub type AsyncParentFn<'a> = dyn FnMut() -> Path + Send + 'a;
/// The core trait of this crate.
///
/// Validation runs the fields through every validation rules,
/// and aggregates any errors into a [`Report`].
pub trait AsyncValidate: Send + Sync {
    /// A user-provided context.
    ///
    /// Custom validators receive a reference to this context.
    type Context: Send + Sync;

    /// Validates `Self`, returning an `Err` with an aggregate of all errors if
    /// the validation failed.
    ///
    /// This method should not be implemented manually. Implement [`AsyncValidate::validate_into`] instead,
    /// because [`AsyncValidate::validate`] has a default implementation that calls [`AsyncValidate::validate_into`].
    fn validate(&self) -> impl Future<Output = Result<(), Report>> + Send
    where
        Self::Context: Default,
    {
        async {
            let ctx = Self::Context::default();
            self.validate_with(&ctx).await
        }
    }

    /// Validates `Self`, returning an `Err` with an aggregate of all errors if
    /// the validation failed.
    ///
    /// This method should not be implemented manually. Implement [`AsyncValidate::validate_into`] instead,
    /// because [`AsyncValidate::validate_with`] has a default implementation that calls [`AsyncValidate::validate_into`].
    fn validate_with(
        &self,
        ctx: &Self::Context,
    ) -> impl Future<Output = Result<(), Report>> + Send {
        async {
            let mut report = Report::new();
            self.validate_into(ctx, &mut Path::empty, &mut report).await;
            match report.is_empty() {
                true => Ok(()),
                false => Err(report),
            }
        }
    }

    /// Validates `Self`, aggregating all validation errors into `Report`.
    fn validate_into(
        &self,
        ctx: &Self::Context,
        parent: &mut AsyncParentFn,
        report: &mut Report,
    ) -> impl Future<Output = ()> + Send;
}

/// A struct which wraps a potentially invalid instance of some `T`.
///
/// Use the `validate` method to turn this type into a `Valid<T>`.
#[derive(Clone, Copy, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[repr(transparent)]
pub struct AsyncUnvalidated<T>(T);

impl<T: AsyncValidate> AsyncUnvalidated<T> {
    /// Creates an `AsyncUnvalidate<T>`
    pub fn new(v: T) -> Self {
        Self(v)
    }

    /// Validates `self`, transforming it into a `Valid<T>`.
    /// This is the only way to create an instance of `Valid<T>`.
    pub async fn validate(self) -> Result<Valid<T>, Report>
    where
        <T as AsyncValidate>::Context: Default,
    {
        self.0.validate().await?;
        Ok(Valid(self.0))
    }

    /// Validates `self`, transforming it into a `Valid<T>`.
    /// This is the only way to create an instance of `Valid<T>`.
    pub async fn validate_with(
        self,
        ctx: &<T as AsyncValidate>::Context,
    ) -> Result<Valid<T>, Report> {
        self.0.validate_with(ctx).await?;
        Ok(Valid(self.0))
    }
}

impl<T: AsyncValidate> From<T> for AsyncUnvalidated<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: Debug> Debug for AsyncUnvalidated<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl<T: ?Sized + AsyncValidate> AsyncValidate for &T {
    type Context = T::Context;

    async fn validate_into(
        &self,
        ctx: &Self::Context,
        parent: &mut AsyncParentFn<'_>,
        report: &mut Report,
    ) {
        <T as AsyncValidate>::validate_into(self, ctx, parent, report).await
    }
}

impl<T: ?Sized + AsyncValidate> AsyncValidate for &mut T {
    type Context = T::Context;

    async fn validate_into(
        &self,
        ctx: &Self::Context,
        parent: &mut AsyncParentFn<'_>,
        report: &mut Report,
    ) {
        <T as AsyncValidate>::validate_into(self, ctx, parent, report).await
    }
}

impl<T: AsyncValidate> AsyncValidate for std::boxed::Box<T> {
    type Context = T::Context;

    async fn validate_into(
        &self,
        ctx: &Self::Context,
        parent: &mut AsyncParentFn<'_>,
        report: &mut Report,
    ) {
        <T as AsyncValidate>::validate_into(self, ctx, parent, report).await
    }
}
impl<T: AsyncValidate> AsyncValidate for std::sync::Arc<T> {
    type Context = T::Context;

    async fn validate_into(
        &self,
        ctx: &Self::Context,
        parent: &mut AsyncParentFn<'_>,
        report: &mut Report,
    ) {
        <T as AsyncValidate>::validate_into(self, ctx, parent, report).await
    }
}

impl AsyncValidate for () {
    type Context = ();

    async fn validate_into(&self, _: &Self::Context, _: &mut AsyncParentFn<'_>, _: &mut Report) {}
}
impl<T: AsyncValidate> AsyncValidate for Option<T> {
    type Context = T::Context;

    async fn validate_into(
        &self,
        ctx: &Self::Context,
        parent: &mut AsyncParentFn<'_>,
        report: &mut Report,
    ) {
        if let Some(value) = self {
            value.validate_into(ctx, parent, report).await
        }
    }
}
