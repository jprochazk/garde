//! ## Core validation traits and types

use std::fmt::Debug;

use crate::error::{Path, PathComponentKind};
use crate::Report;

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
    fn validate(&self, ctx: &Self::Context) -> Result<(), Report> {
        let mut report = Report::new();
        self.validate_into(ctx, &Path::empty(), &mut report);
        match report.is_empty() {
            true => Ok(()),
            false => Err(report),
        }
    }

    fn validate_into(&self, ctx: &Self::Context, current_path: &Path, report: &mut Report);
}

/// A struct which wraps a valid instance of some `T`.
///
/// The only way to create an instance of this struct is through the `validate`
/// function on the [`Unvalidated`] type. This ensures that if you have a `Valid<T>`,
/// it was definitely validated at some point. This is commonly referred to as the
/// typestate pattern.
#[derive(Debug, Clone, Copy)]
pub struct Valid<T>(T);

impl<T: Validate> Valid<T> {
    /// Returns the inner value.
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
#[cfg_attr(feature = "serde", serde(transparent))]
#[repr(transparent)]
pub struct Unvalidated<T>(T);

impl<T: Validate> Unvalidated<T> {
    /// Creates an `Unvalidated<T>`
    pub fn new(v: T) -> Self {
        Self(v)
    }

    /// Validates `self`, transforming it into a `Valid<T>`.
    /// This is the only way to create an instance of `Valid<T>`.
    pub fn validate(self, ctx: &<T as Validate>::Context) -> Result<Valid<T>, Report> {
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

impl<'a, T: ?Sized + Validate> Validate for &'a T {
    type Context = T::Context;

    fn validate_into(&self, ctx: &Self::Context, current_path: &Path, report: &mut Report) {
        <T as Validate>::validate_into(self, ctx, current_path, report)
    }
}

impl<'a, T: ?Sized + Validate> Validate for &'a mut T {
    type Context = T::Context;

    fn validate_into(&self, ctx: &Self::Context, current_path: &Path, report: &mut Report) {
        <T as Validate>::validate_into(self, ctx, current_path, report)
    }
}

impl<T: Validate> Validate for std::boxed::Box<T> {
    type Context = T::Context;

    fn validate_into(&self, ctx: &Self::Context, current_path: &Path, report: &mut Report) {
        <T as Validate>::validate_into(self, ctx, current_path, report)
    }
}

impl<T: Validate> Validate for std::rc::Rc<T> {
    type Context = T::Context;

    fn validate_into(&self, ctx: &Self::Context, current_path: &Path, report: &mut Report) {
        <T as Validate>::validate_into(self, ctx, current_path, report)
    }
}

impl<T: Validate> Validate for std::sync::Arc<T> {
    type Context = T::Context;

    fn validate_into(&self, ctx: &Self::Context, current_path: &Path, report: &mut Report) {
        <T as Validate>::validate_into(self, ctx, current_path, report)
    }
}

macro_rules! impl_validate_list {
    (<$T:ident $(, $Other:ident)*> $Container:ty) => {
        impl<$T, $($Other),*> Validate for $Container
        where
            $T: Validate
        {
            type Context = T::Context;

            fn validate_into(&self, ctx: &Self::Context, current_path: &Path, report: &mut Report) {
                for (index, item) in self.iter().enumerate() {
                    <T as Validate>::validate_into(item, ctx, &current_path.join(index), report);
                }
            }
        }
    };
}

impl_validate_list!(<T, S> std::collections::HashSet<T, S>);
impl_validate_list!(<T> std::collections::BTreeSet<T>);
impl_validate_list!(<T> std::collections::BinaryHeap<T>);
impl_validate_list!(<T> std::collections::LinkedList<T>);
impl_validate_list!(<T> std::collections::VecDeque<T>);
impl_validate_list!(<T> std::vec::Vec<T>);
impl_validate_list!(<T> [T]);

impl<T: Validate, const N: usize> Validate for [T; N] {
    type Context = T::Context;

    fn validate_into(&self, ctx: &Self::Context, current_path: &Path, report: &mut Report) {
        for (index, item) in self.iter().enumerate() {
            <T as Validate>::validate_into(item, ctx, &current_path.join(index), report);
        }
    }
}

macro_rules! impl_validate_tuple {
    ($A:ident, $($T:ident),*) => {
        impl<$A, $($T),*> Validate for ($A, $($T,)*)
        where
            $A : Validate,
            $($T : Validate<Context=$A::Context>,)*
        {
            type Context = $A::Context;

            #[allow(non_snake_case)]
            fn validate_into(&self, ctx: &Self::Context, current_path: &Path, report: &mut Report) {
                let ($A, $($T,)*) = self;
                let mut index = 0;
                <$A as Validate>::validate_into($A, ctx, &current_path.join(index), report);
                index += 1;
                $({
                    <$T as Validate>::validate_into($T, ctx, &current_path.join(index), report);
                    index += 1;
                })*
                let _ = index;
            }
        }
    }
}

impl_validate_tuple!(A,);
impl_validate_tuple!(A, B);
impl_validate_tuple!(A, B, C);
impl_validate_tuple!(A, B, C, D);
impl_validate_tuple!(A, B, C, D, E);
impl_validate_tuple!(A, B, C, D, E, F);
impl_validate_tuple!(A, B, C, D, E, F, G);
impl_validate_tuple!(A, B, C, D, E, F, G, H);
impl_validate_tuple!(A, B, C, D, E, F, G, H, I);
impl_validate_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_validate_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_validate_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);

impl Validate for () {
    type Context = ();

    fn validate_into(&self, _: &Self::Context, _: &Path, _: &mut Report) {}
}

impl<K, V, S> Validate for std::collections::HashMap<K, V, S>
where
    std::borrow::Cow<'static, str>: From<K>,
    K: Clone + PathComponentKind,
    V: Validate,
{
    type Context = V::Context;

    fn validate_into(&self, ctx: &Self::Context, current_path: &Path, report: &mut Report) {
        for (key, value) in self.iter() {
            <V as Validate>::validate_into(value, ctx, &current_path.join(key.clone()), report);
        }
    }
}

impl<K, V> Validate for std::collections::BTreeMap<K, V>
where
    std::borrow::Cow<'static, str>: From<K>,
    K: Clone + PathComponentKind,
    V: Validate,
{
    type Context = V::Context;

    fn validate_into(&self, ctx: &Self::Context, current_path: &Path, report: &mut Report) {
        for (key, value) in self.iter() {
            <V as Validate>::validate_into(value, ctx, &current_path.join(key.clone()), report);
        }
    }
}

impl<T: Validate> Validate for Option<T> {
    type Context = T::Context;

    fn validate_into(&self, ctx: &Self::Context, current_path: &Path, report: &mut Report) {
        if let Some(value) = self {
            value.validate_into(ctx, current_path, report)
        }
    }
}
