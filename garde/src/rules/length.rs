//! Length validation.
//!
//! ```rust
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(length(min=1, max=100))]
//!     v: String,
//! }
//! ```
//!
//! The entrypoint is the [`Length`] trait. Implementing this trait for a type allows that type to be used with the `#[garde(length(...))]` rule.
//!
//! The [`Length`] has a companion trait [`HasLength`], which may be implemented for any container with a known length.
//! [`Length`] is implemented for any `T: HasLength`.
//!
//! In case of string types, [`HasLength::length`] should return the number of _characters_ as opposed to the number of _bytes_.
//! For validation of length counted in _bytes_, see the [`crate::rules::byte_length`] rule.
//!
//! Here's what implementing the trait for a custom string-like type might look like:
//! ```rust
//! #[repr(transparent)]
//! struct MyString(String);
//!
//! impl garde::rules::length::HasLength for MyString {
//!     fn length(&self) -> usize {
//!         self.0.chars().count()
//!     }
//! }
//! ```
//!

use crate::error::Error;

pub fn apply<T: Length>(v: &T, (min, max): (usize, usize)) -> Result<(), Error> {
    if let Err(e) = v.validate_length(min, max) {
        match e {
            InvalidLength::Min => return Err(Error::new(format!("length is lower than {min}"))),
            InvalidLength::Max => return Err(Error::new(format!("length is greater than {max}"))),
        }
    }
    Ok(())
}

pub trait Length {
    fn validate_length(&self, min: usize, max: usize) -> Result<(), InvalidLength>;
}

pub enum InvalidLength {
    Min,
    Max,
}

#[allow(clippy::len_without_is_empty)]
pub trait HasLength {
    fn length(&self) -> usize;
}

impl<T: HasLength> Length for T {
    fn validate_length(&self, min: usize, max: usize) -> Result<(), InvalidLength> {
        let len = HasLength::length(self);
        if len < min {
            Err(InvalidLength::Min)
        } else if len > max {
            Err(InvalidLength::Max)
        } else {
            Ok(())
        }
    }
}

impl HasLength for String {
    fn length(&self) -> usize {
        self.chars().count()
    }
}

impl<'a> HasLength for &'a String {
    fn length(&self) -> usize {
        self.chars().count()
    }
}

impl<'a> HasLength for &'a str {
    fn length(&self) -> usize {
        self.chars().count()
    }
}

impl<'a> HasLength for std::borrow::Cow<'a, str> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T> HasLength for Vec<T> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<'a, T> HasLength for &'a Vec<T> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T> HasLength for &[T] {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T, const N: usize> HasLength for [T; N] {
    fn length(&self) -> usize {
        N
    }
}

impl<T, const N: usize> HasLength for &[T; N] {
    fn length(&self) -> usize {
        N
    }
}

impl<'a, K, V, S> HasLength for &'a std::collections::HashMap<K, V, S> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<K, V, S> HasLength for std::collections::HashMap<K, V, S> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<'a, T, S> HasLength for &'a std::collections::HashSet<T, S> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T, S> HasLength for std::collections::HashSet<T, S> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<'a, K, V> HasLength for &'a std::collections::BTreeMap<K, V> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<K, V> HasLength for std::collections::BTreeMap<K, V> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<'a, T> HasLength for &'a std::collections::BTreeSet<T> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T> HasLength for std::collections::BTreeSet<T> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T> HasLength for std::collections::VecDeque<T> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T> HasLength for std::collections::BinaryHeap<T> {
    fn length(&self) -> usize {
        self.len()
    }
}

impl<T> HasLength for std::collections::LinkedList<T> {
    fn length(&self) -> usize {
        self.len()
    }
}
