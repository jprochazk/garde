//! Byte length validation.
//!
//! ```rust
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(byte_length(min=1, max=100))]
//!     v: String,
//! }
//! ```
//!
//! The entrypoint is the [`ByteLength`] trait. Implementing this trait for a type allows that type to be used with the `#[garde(byte_length(...))]` rule.
//!
//! The [`ByteLength`] has a companion trait [`HasByteLength`], which may be implemented for any container with a known length counted in bytes.
//! [`ByteLength`] is implemented for any `T: HasByteLength`.
//!
//! In case of string types, [`HasByteLength::byte_length`] should return the number of _bytes_ as opposed to the number of _characters_.
//! For validation of length counted in _characters_, see the [`crate::rules::length`] rule.
//!
//! Here's what implementing the trait for a custom string-like type might look like:
//! ```rust
//! #[repr(transparent)]
//! struct MyString(String);
//!
//! impl garde::rules::byte_length::HasByteLength for MyString {
//!     fn byte_length(&self) -> usize {
//!         self.0.len()
//!     }
//! }
//! ```

use super::AsStr;
use crate::error::Error;

#[deprecated = "the `byte_length` attribute is deprecated. Use `length` instead. (See https://github.com/jprochazk/garde/issues/84)"]
pub fn apply<T: ByteLength>(v: &T, (min, max): (usize, usize)) -> Result<(), Error> {
    if let Err(e) = v.validate_byte_length(min, max) {
        match e {
            InvalidLength::Min => {
                return Err(Error::new(format!("byte length is lower than {min}")))
            }
            InvalidLength::Max => {
                return Err(Error::new(format!("byte length is greater than {max}")))
            }
        }
    }
    Ok(())
}

pub trait ByteLength {
    fn validate_byte_length(&self, min: usize, max: usize) -> Result<(), InvalidLength>;
}

pub enum InvalidLength {
    Min,
    Max,
}

#[allow(clippy::len_without_is_empty)]
pub trait HasByteLength {
    fn byte_length(&self) -> usize;
}

impl<T: HasByteLength> ByteLength for T {
    fn validate_byte_length(&self, min: usize, max: usize) -> Result<(), InvalidLength> {
        let len = HasByteLength::byte_length(self);
        if len < min {
            Err(InvalidLength::Min)
        } else if len > max {
            Err(InvalidLength::Max)
        } else {
            Ok(())
        }
    }
}

impl<T: ByteLength> ByteLength for Option<T> {
    fn validate_byte_length(&self, min: usize, max: usize) -> Result<(), InvalidLength> {
        match self {
            Some(value) => value.validate_byte_length(min, max),
            None => Ok(()),
        }
    }
}

impl<T: AsByteSlice> HasByteLength for T {
    fn byte_length(&self) -> usize {
        self.as_byte_slice().len()
    }
}

pub trait AsByteSlice {
    fn as_byte_slice(&self) -> &[u8];
}

impl<'a> AsByteSlice for &'a [u8] {
    fn as_byte_slice(&self) -> &[u8] {
        self
    }
}

impl AsByteSlice for Vec<u8> {
    fn as_byte_slice(&self) -> &[u8] {
        self.as_slice()
    }
}

impl<const N: usize> AsByteSlice for [u8; N] {
    fn as_byte_slice(&self) -> &[u8] {
        self
    }
}

impl<T: AsStr> AsByteSlice for T {
    fn as_byte_slice(&self) -> &[u8] {
        self.as_str().as_bytes()
    }
}
