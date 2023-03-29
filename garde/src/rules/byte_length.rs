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

use crate::error::Error;

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

#[cfg_attr(
    feature = "nightly-error-messages",
    rustc_on_unimplemented(
        message = "`{Self}` does not support byte length validation",
        label = "This type does not support byte length validation",
        note = "try implementing `garde::rules::length::HasByteLength` for `{Self}`"
    )
)]
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

impl<T: AsRef<[u8]>> HasByteLength for T {
    fn byte_length(&self) -> usize {
        self.as_ref().len()
    }
}
