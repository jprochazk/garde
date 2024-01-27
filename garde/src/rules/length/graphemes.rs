//! Implemented by string-like types for which we can retrieve length in the number of graphemes.
//!
//! `garde` implementations of this trait use the [unicode-segmentation](https://crates.io/crates/unicode-segmentation) crate.

use crate::error::Error;

pub fn apply<T: Graphemes>(v: &T, (min, max): (usize, usize)) -> Result<(), Error> {
    v.validate_num_graphemes(min, max)
}

pub trait Graphemes {
    fn validate_num_graphemes(&self, min: usize, max: usize) -> Result<(), Error>;
}

impl<T: HasGraphemes> Graphemes for T {
    fn validate_num_graphemes(&self, min: usize, max: usize) -> Result<(), Error> {
        super::check_len(self.num_graphemes(), min, max)
    }
}

impl<T: Graphemes> Graphemes for Option<T> {
    fn validate_num_graphemes(&self, min: usize, max: usize) -> Result<(), Error> {
        match self {
            Some(v) => v.validate_num_graphemes(min, max),
            None => Ok(()),
        }
    }
}

pub trait HasGraphemes {
    fn num_graphemes(&self) -> usize;
}

macro_rules! impl_str {
    ($(in<$lifetime:lifetime>)? $T:ty) => {
        impl<$($lifetime)?> HasGraphemes for $T {
            fn num_graphemes(&self) -> usize {
                use unicode_segmentation::UnicodeSegmentation;

                self.graphemes(true).count()
            }
        }
    };
}

impl_str!(std::string::String);
impl_str!(in<'a> &'a str);
impl_str!(in<'a> std::borrow::Cow<'a, str>);
impl_str!(std::rc::Rc<str>);
impl_str!(std::sync::Arc<str>);
impl_str!(std::boxed::Box<str>);
