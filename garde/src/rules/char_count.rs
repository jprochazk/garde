//! Character count validation. Works as `string.chars().count()` and counts **USVs** (Unicode Scalar Value).
//!
//! It's important to remember that `char` represents a **USV**, and [may not match your idea](https://stackoverflow.com/a/46290728) of
//! what a 'character' is. Using grapheme clusters (the [`crate::rules::grapheme_count`] rule) may be what you actually want.
//!
//! ```rust
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(char_count(min=1, max=100))]
//!     v: String,
//! }
//! ```
//!
//! The entrypoint is the [`CharCount`] trait. Implementing this trait for a type allows that type to be used with the `#[garde(char_count(...))]` rule.
//!
//! For validation of length counted in _bytes_, see the [`crate::rules::length`] rule.
//!
//! Here's what implementing the trait for a custom string-like type might look like:
//! ```rust
//! #[repr(transparent)]
//! struct MyString(String);
//!
//! impl garde::rules::char_count::HasCharCount for MyString {
//!     fn char_count(&self) -> usize {
//!         self.0.chars().count()
//!     }
//! }
//! ```

use crate::error::Error;

pub fn apply<T: CharCount>(v: &T, (min, max): (usize, usize)) -> Result<(), Error> {
    if let Err(e) = v.validate_char_count(min, max) {
        match e {
            InvalidLength::Min => {
                return Err(Error::new(format!("character count is lower than {min}")))
            }
            InvalidLength::Max => {
                return Err(Error::new(format!("character count is greater than {max}")))
            }
        }
    }
    Ok(())
}

pub trait CharCount {
    fn validate_char_count(&self, min: usize, max: usize) -> Result<(), InvalidLength>;
}

pub enum InvalidLength {
    Min,
    Max,
}

#[allow(clippy::len_without_is_empty)]
pub trait HasCharCount {
    fn char_count(&self) -> usize;
}

impl<T: HasCharCount> CharCount for T {
    fn validate_char_count(&self, min: usize, max: usize) -> Result<(), InvalidLength> {
        let len = HasCharCount::char_count(self);
        if len < min {
            Err(InvalidLength::Min)
        } else if len > max {
            Err(InvalidLength::Max)
        } else {
            Ok(())
        }
    }
}

impl<T: CharCount> CharCount for Option<T> {
    fn validate_char_count(&self, min: usize, max: usize) -> Result<(), InvalidLength> {
        match self {
            Some(value) => value.validate_char_count(min, max),
            None => Ok(()),
        }
    }
}

impl HasCharCount for String {
    fn char_count(&self) -> usize {
        self.chars().count()
    }
}

impl<'a> HasCharCount for &'a String {
    fn char_count(&self) -> usize {
        self.chars().count()
    }
}

impl<'a> HasCharCount for &'a str {
    fn char_count(&self) -> usize {
        self.chars().count()
    }
}

impl<'a> HasCharCount for std::borrow::Cow<'a, str> {
    fn char_count(&self) -> usize {
        self.chars().count()
    }
}

impl<'a, 'b> HasCharCount for &'a std::borrow::Cow<'b, str> {
    fn char_count(&self) -> usize {
        self.chars().count()
    }
}

impl HasCharCount for Box<str> {
    fn char_count(&self) -> usize {
        self.chars().count()
    }
}

impl<'a> HasCharCount for &'a Box<str> {
    fn char_count(&self) -> usize {
        self.chars().count()
    }
}
