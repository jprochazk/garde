//! Implemented by types which have a known length.
//!
//! The meaning of "length" depends on the type.
//! For example, the length of a `String` is defined as the number of _bytes_ it stores.

use crate::error::Error;

pub fn apply<T: Simple>(v: &T, (min, max): (usize, usize)) -> Result<(), Error> {
    v.validate_length(min, max)
}

pub trait Simple {
    fn validate_length(&self, min: usize, max: usize) -> Result<(), Error>;
}

impl<T: HasSimpleLength> Simple for T {
    fn validate_length(&self, min: usize, max: usize) -> Result<(), Error> {
        super::check_len(self.length(), min, max)
    }
}

impl<T: Simple> Simple for Option<T> {
    fn validate_length(&self, min: usize, max: usize) -> Result<(), Error> {
        match self {
            Some(v) => v.validate_length(min, max),
            None => Ok(()),
        }
    }
}

pub trait HasSimpleLength {
    fn length(&self) -> usize;
}

macro_rules! impl_via_bytes {
    ($(in<$lifetime:lifetime>)? $T:ty) => {
        impl<$($lifetime)?> HasSimpleLength for $T {
            fn length(&self) -> usize {
                use super::bytes::HasBytes as _;
                self.num_bytes()
            }
        }
    };
}

impl_via_bytes!(std::string::String);
impl_via_bytes!(in<'a> &'a str);
impl_via_bytes!(in<'a> std::borrow::Cow<'a, str>);
impl_via_bytes!(std::rc::Rc<str>);
impl_via_bytes!(std::sync::Arc<str>);
impl_via_bytes!(std::boxed::Box<str>);

macro_rules! impl_via_len {
    (in<$lifetime:lifetime, $($generic:ident),*> $T:ty) => {
        impl<$lifetime, $($generic),*> HasSimpleLength for $T {
            fn length(&self) -> usize {
                self.len()
            }
        }
    };
    (in<$($generic:ident),*> $T:ty) => {
        impl<$($generic),*> HasSimpleLength for $T {
            fn length(&self) -> usize {
                self.len()
            }
        }
    };
    (in<$lifetime:lifetime> $T:ty) => {
        impl<$lifetime> HasSimpleLength for $T {
            fn length(&self) -> usize {
                self.len()
            }
        }
    };
    ($T:ty) => {
        impl HasSimpleLength for $T {
            fn length(&self) -> usize {
                self.len()
            }
        }
    };
}

impl_via_len!(in<T> Vec<T>);
impl_via_len!(in<'a, T> &'a [T]);

impl<const N: usize, T> Simple for [T; N] {
    fn validate_length(&self, min: usize, max: usize) -> Result<(), Error> {
        super::check_len(self.len(), min, max)
    }
}

impl<'a, const N: usize, T> Simple for &'a [T; N] {
    fn validate_length(&self, min: usize, max: usize) -> Result<(), Error> {
        super::check_len(self.len(), min, max)
    }
}

impl_via_len!(in<K, V, S> std::collections::HashMap<K, V, S>);
impl_via_len!(in<T, S> std::collections::HashSet<T, S>);
impl_via_len!(in<K, V> std::collections::BTreeMap<K, V>);
impl_via_len!(in<T> std::collections::BTreeSet<T>);
impl_via_len!(in<T> std::collections::VecDeque<T>);
impl_via_len!(in<T> std::collections::BinaryHeap<T>);
impl_via_len!(in<T> std::collections::LinkedList<T>);
