//! Grapheme count validation using the [`unicode_segmentation`] crate.
//!
//! ```rust
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(grapheme_count(min=1, max=100))]
//!     v: String,
//! }
//! ```
//!
//! The entrypoint is the [`GraphemeCount`] trait. Implementing this trait for a type allows that type to be used with the `#[garde(grapheme_count(...))]` rule.
//!
//! Here's what implementing the trait for a custom string-like type might look like:
//! ```rust
//! use unicode_segmentation::UnicodeSegmentation;
//!
//! #[repr(transparent)]
//! struct MyString(String);
//!
//! impl garde::rules::grapheme_count::HasGraphemeCount for MyString {
//!     fn grapheme_count(&self) -> usize {
//!         self.0.graphemes(true).count()
//!     }
//! }
//! ```
//!

use unicode_segmentation::UnicodeSegmentation;

use crate::error::Error;

pub fn apply<T: GraphemeCount>(v: &T, (min, max): (usize, usize)) -> Result<(), Error> {
    if let Err(e) = v.validate_grapheme_count(min, max) {
        match e {
            InvalidLength::Min => {
                return Err(Error::new(format!("grapheme count is lower than {min}")))
            }
            InvalidLength::Max => {
                return Err(Error::new(format!("grapheme count is greater than {max}")))
            }
        }
    }
    Ok(())
}

pub trait GraphemeCount {
    fn validate_grapheme_count(&self, min: usize, max: usize) -> Result<(), InvalidLength>;
}

pub enum InvalidLength {
    Min,
    Max,
}

#[allow(clippy::len_without_is_empty)]
pub trait HasGraphemeCount {
    fn grapheme_count(&self) -> usize;
}

impl<T: HasGraphemeCount> GraphemeCount for T {
    fn validate_grapheme_count(&self, min: usize, max: usize) -> Result<(), InvalidLength> {
        let len = HasGraphemeCount::grapheme_count(self);
        if len < min {
            Err(InvalidLength::Min)
        } else if len > max {
            Err(InvalidLength::Max)
        } else {
            Ok(())
        }
    }
}

impl<T: GraphemeCount> GraphemeCount for Option<T> {
    fn validate_grapheme_count(&self, min: usize, max: usize) -> Result<(), InvalidLength> {
        match self {
            Some(value) => value.validate_grapheme_count(min, max),
            None => Ok(()),
        }
    }
}

impl HasGraphemeCount for String {
    fn grapheme_count(&self) -> usize {
        self.graphemes(true).count()
    }
}

impl<'a> HasGraphemeCount for &'a String {
    fn grapheme_count(&self) -> usize {
        self.graphemes(true).count()
    }
}

impl<'a> HasGraphemeCount for &'a str {
    fn grapheme_count(&self) -> usize {
        self.graphemes(true).count()
    }
}

impl<'a> HasGraphemeCount for std::borrow::Cow<'a, str> {
    fn grapheme_count(&self) -> usize {
        self.graphemes(true).count()
    }
}

impl<'a, 'b> HasGraphemeCount for &'a std::borrow::Cow<'b, str> {
    fn grapheme_count(&self) -> usize {
        self.graphemes(true).count()
    }
}

impl HasGraphemeCount for Box<str> {
    fn grapheme_count(&self) -> usize {
        self.graphemes(true).count()
    }
}

impl<'a> HasGraphemeCount for &'a Box<str> {
    fn grapheme_count(&self) -> usize {
        self.graphemes(true).count()
    }
}
