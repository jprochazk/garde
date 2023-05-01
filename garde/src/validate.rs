//! ## Core validation traits and types

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

    /// Consumes and validates `self`, returning it un-touched if it
    /// passes validation or the provided `other` version on failure.
    fn validate_or(self, other: Self, ctx: &Self::Context) -> Self
    where
        Self: Sized,
    {
        match self.validate(ctx) {
            Ok(_) => self,
            Err(_) => other,
        }
    }

    /// Consumes and validates `self`, returning it un-touched if it
    /// passes validation or computes a new `Self` from closure `f` on failure.
    fn validate_or_else<F>(self, f: F, ctx: &Self::Context) -> Self
    where
        Self: Sized,
        F: FnOnce() -> Self,
    {
        match self.validate(ctx) {
            Ok(_) => self,
            Err(_) => f(),
        }
    }

    /// Consumes and validates `self`, returning it un-touched if it
    /// passes validation or a [`Default`] version on failure.
    fn validate_or_default(self, ctx: &Self::Context) -> Self
    where
        Self: Default,
    {
        match self.validate(ctx) {
            Ok(_) => self,
            Err(_) => Default::default(),
        }
    }
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
pub struct Unvalidated<T>(T);

impl<T: Validate> Unvalidated<T> {
    /// Creates an `Unvalidated<T>`
    pub fn new(v: T) -> Self {
        Self(v)
    }

    /// Validates `self`, transforming it into a `Valid<T>`.
    /// This is the only way to create an instance of `Valid<T>`.
    pub fn validate(self, ctx: &<T as Validate>::Context) -> Result<Valid<T>, Errors> {
        self.0.validate(ctx)?;
        Ok(Valid(self.0))
    }

    /// Validates `self`, wrapped in [`Valid`].
    ///
    /// - `self` is returned if it passes validation
    /// - the provided `other` is returned on failure
    pub fn validate_or(self, other: T, ctx: &<T as Validate>::Context) -> Valid<T> {
        match self.0.validate(ctx) {
            Ok(_) => Valid(self.0),
            Err(_) => Valid(other),
        }
    }

    /// Validates `self`, wrapped in [`Valid`].
    ///
    /// - `self` is returned if it passes validation
    /// - the output of closure `f` is returned on failure
    pub fn validate_or_else<F>(self, f: F, ctx: &<T as Validate>::Context) -> Valid<T>
    where
        F: FnOnce() -> T,
    {
        match self.0.validate(ctx) {
            Ok(_) => Valid(self.0),
            Err(_) => Valid(f()),
        }
    }

    /// Validates `self`, transforming it into a `Valid<T>` on sucesss
    /// or a `Valid<T>` with [`Default`] on failure.
    pub fn validate_or_default(self, ctx: &<T as Validate>::Context) -> Valid<T>
    where
        T: Default,
    {
        match self.0.validate(ctx) {
            Ok(_) => Valid(self.0),
            Err(_) => Valid(Default::default()),
        }
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

    fn validate(&self, ctx: &Self::Context) -> Result<(), Errors> {
        <T as Validate>::validate(self, ctx)
    }
}

impl<'a, T: ?Sized + Validate> Validate for &'a mut T {
    type Context = T::Context;

    fn validate(&self, ctx: &Self::Context) -> Result<(), Errors> {
        <T as Validate>::validate(self, ctx)
    }
}

impl<T: Validate> Validate for std::boxed::Box<T> {
    type Context = T::Context;

    fn validate(&self, ctx: &Self::Context) -> Result<(), Errors> {
        <T as Validate>::validate(self, ctx)
    }
}

impl<T: Validate> Validate for std::rc::Rc<T> {
    type Context = T::Context;

    fn validate(&self, ctx: &Self::Context) -> Result<(), Errors> {
        <T as Validate>::validate(self, ctx)
    }
}

impl<T: Validate> Validate for std::sync::Arc<T> {
    type Context = T::Context;

    fn validate(&self, ctx: &Self::Context) -> Result<(), Errors> {
        <T as Validate>::validate(self, ctx)
    }
}

macro_rules! impl_validate_list {
    (<$T:ident $(, $Other:ident)*> $Container:ty) => {
        impl<$T, $($Other),*> Validate for $Container
        where
            $T: Validate
        {
            type Context = T::Context;
            fn validate(&self, ctx: &Self::Context) -> Result<(), Errors> {
                let errors = Errors::list(|errors| {
                    for item in self.iter() {
                        errors.push(
                            <T as Validate>::validate(item, ctx)
                                .err()
                                .unwrap_or_else(Errors::empty),
                        )
                    }
                });
                if !errors.is_empty() {
                    return Err(errors);
                }
                Ok(())
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
    fn validate(&self, ctx: &Self::Context) -> Result<(), Errors> {
        let errors = Errors::list(|errors| {
            for item in self.iter() {
                errors.push(
                    <T as Validate>::validate(item, ctx)
                        .err()
                        .unwrap_or_else(Errors::empty),
                )
            }
        });
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(())
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
            fn validate(&self, ctx: &Self::Context) -> Result<(), Errors> {
                let errors = Errors::list(|errors| {
                    let ($A, $($T,)*) = self;
                    errors.push(
                        <$A as Validate>::validate($A, ctx)
                            .err()
                            .unwrap_or_else(|| Errors::empty())
                    );
                    $(
                        errors.push(
                            <$T as Validate>::validate($T, ctx)
                                .err()
                                .unwrap_or_else(|| Errors::empty())
                        );
                    )*
                });
                if !errors.is_empty() {
                    return Err(errors);
                }
                Ok(())
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

    fn validate(&self, _: &Self::Context) -> Result<(), Errors> {
        Ok(())
    }
}

impl<K, V, S> Validate for std::collections::HashMap<K, V, S>
where
    std::borrow::Cow<'static, str>: From<K>,
    K: Clone,
    V: Validate,
{
    type Context = V::Context;

    fn validate(&self, ctx: &Self::Context) -> Result<(), Errors> {
        let errors = Errors::fields(|errors| {
            for (key, value) in self.iter() {
                errors.insert(
                    std::borrow::Cow::from(key.clone()),
                    <V as Validate>::validate(value, ctx)
                        .err()
                        .unwrap_or_else(Errors::empty),
                )
            }
        });
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(())
    }
}

impl<K, V> Validate for std::collections::BTreeMap<K, V>
where
    std::borrow::Cow<'static, str>: From<K>,
    K: Clone,
    V: Validate,
{
    type Context = V::Context;

    fn validate(&self, ctx: &Self::Context) -> Result<(), Errors> {
        let errors = Errors::fields(|errors| {
            for (key, value) in self.iter() {
                errors.insert(
                    std::borrow::Cow::from(key.clone()),
                    <V as Validate>::validate(value, ctx)
                        .err()
                        .unwrap_or_else(Errors::empty),
                )
            }
        });
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(())
    }
}
