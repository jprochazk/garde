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

use crate::error::{NoKey, PathComponentKind};
use std::future::Future;

pub fn apply<T, U, K, F>(field: &T, f: F)
where
    T: Inner<U, Key = K>,
    F: FnMut(&U, &K),
{
    field.validate_inner(f)
}

pub trait Inner<T> {
    type Key: PathComponentKind;

    fn validate_inner<F>(&self, f: F)
    where
        F: FnMut(&T, &Self::Key);
}

impl<T> Inner<T> for Vec<T> {
    type Key = usize;

    fn validate_inner<F>(&self, f: F)
    where
        F: FnMut(&T, &Self::Key),
    {
        self.as_slice().validate_inner(f)
    }
}

impl<const N: usize, T> Inner<T> for [T; N] {
    type Key = usize;

    fn validate_inner<F>(&self, f: F)
    where
        F: FnMut(&T, &Self::Key),
    {
        self.as_slice().validate_inner(f)
    }
}

impl<T> Inner<T> for &[T] {
    type Key = usize;

    fn validate_inner<F>(&self, mut f: F)
    where
        F: FnMut(&T, &Self::Key),
    {
        for (index, item) in self.iter().enumerate() {
            f(item, &index);
        }
    }
}

impl<T> Inner<T> for Option<T> {
    type Key = NoKey;

    fn validate_inner<F>(&self, mut f: F)
    where
        F: FnMut(&T, &Self::Key),
    {
        if let Some(item) = self {
            f(item, &NoKey::default())
        }
    }
}

pub async fn async_apply<T, U, K, F>(field: &T, f: F)
where
    T: InnerAsync<U, Key = K>,
    F: AsyncFnMut(&U, &K) -> () + Send,
    U: Send,
{
    // field.validate_inner_async(f).await
    //FIX: inner async is not implemented, so panic
    todo!("Inner Async is not supported currently :(");
}

// Async trait variant
pub trait InnerAsync<T: Send>: Send + Sync {
    type Key: PathComponentKind;

    fn validate_inner_async<F, Fut>(&self, f: F) -> impl Future<Output = ()> + Send
    where
        F: FnMut(&T, &Self::Key) -> Fut + Send,
        Fut: Future<Output = ()> + Send;
}

// Implementation for Vec<T>
impl<T: std::marker::Send + std::marker::Sync> InnerAsync<T> for Vec<T> {
    type Key = usize;

    fn validate_inner_async<F, Fut>(&self, f: F) -> impl Future<Output = ()>
    where
        F: FnMut(&T, &Self::Key) -> Fut + std::marker::Send,
        Fut: Future<Output = ()> + std::marker::Send,
    {
        async move { self.as_slice().validate_inner_async(f).await }
    }
}

// Implementation for [T; N]
impl<const N: usize, T: std::marker::Send + std::marker::Sync> InnerAsync<T> for [T; N] {
    type Key = usize;

    fn validate_inner_async<F, Fut>(&self, f: F) -> impl Future<Output = ()> + Send
    where
        F: FnMut(&T, &Self::Key) -> Fut + Send,
        Fut: Future<Output = ()> + Send,
    {
        async move {
            self.as_slice().validate_inner_async(f).await;
        }
    }
}

// Implementation for &[T] - sequential execution
impl<T: std::marker::Sync + std::marker::Send> InnerAsync<T> for &[T] {
    type Key = usize;

    async fn validate_inner_async<F, Fut>(&self, mut f: F)
    where
        F: FnMut(&T, &Self::Key) -> Fut + Send,
        Fut: Future<Output = ()> + Send,
    {
        for (index, item) in self.iter().enumerate() {
            f(item, &index).await;
        }
    }
}

// Implementation for Option<T>
impl<T: std::marker::Sync + std::marker::Send> InnerAsync<T> for Option<T> {
    type Key = NoKey;

    async fn validate_inner_async<F, Fut>(&self, mut f: F)
    where
        F: FnMut(&T, &Self::Key) -> Fut,
        Fut: Future<Output = ()>,
    {
        if let Some(item) = self {
            f(item, &NoKey::default()).await;
        }
    }
}
