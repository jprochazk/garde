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

macro_rules! impl_via_iter {
    (in<const $N:ident, $($lifetime:lifetime,)? $V:ident $(, $S:ident)?> $T:ty) => {
        impl<const $N: usize, $($lifetime,)? $V $(, $S)?> Inner<$V> for $T {
            type Key = usize;

            fn validate_inner<F>(&self, mut f: F)
            where
                F: FnMut(&$V, &Self::Key),
            {
                for (index, item) in self.iter().enumerate() {
                    f(item, &index);
                }
            }
        }
    };
    (in<$($lifetime:lifetime,)? $V:ident $(, $S:ident)?> $T:ty) => {
        impl<$($lifetime,)? $V $(, $S)?> Inner<$V> for $T {
            type Key = usize;

            fn validate_inner<F>(&self, mut f: F)
            where
                F: FnMut(&$V, &Self::Key),
            {
                for (index, item) in self.iter().enumerate() {
                    f(item, &index);
                }
            }
        }
    };
}

impl_via_iter!(in<'a, T> &'a [T]);
impl_via_iter!(in<const N, T> [T; N]);
impl_via_iter!(in<T> Vec<T>);
impl_via_iter!(in<T> std::collections::VecDeque<T>);
impl_via_iter!(in<T> std::collections::BinaryHeap<T>);
impl_via_iter!(in<T> std::collections::LinkedList<T>);
impl_via_iter!(in<T, S> std::collections::HashSet<T, S>);
impl_via_iter!(in<T> std::collections::BTreeSet<T>);

impl<K, V, S> Inner<V> for std::collections::HashMap<K, V, S>
where
    K: PathComponentKind,
{
    type Key = K;

    fn validate_inner<F>(&self, mut f: F)
    where
        F: FnMut(&V, &Self::Key),
    {
        for (key, value) in self.iter() {
            f(value, key)
        }
    }
}

impl<K, V> Inner<V> for std::collections::BTreeMap<K, V>
where
    K: PathComponentKind,
{
    type Key = K;

    fn validate_inner<F>(&self, mut f: F)
    where
        F: FnMut(&V, &Self::Key),
    {
        for (key, value) in self.iter() {
            f(value, key)
        }
    }
}
