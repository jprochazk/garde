//! Implemented by string-like types for which we can retrieve length in _UTF-16 code units_.

use crate::error::Error;

pub fn apply<T: Utf16CodeUnits>(v: &T, (min, max): (usize, usize)) -> Result<(), Error> {
    v.validate_num_code_units(min, max)
}

pub trait Utf16CodeUnits {
    fn validate_num_code_units(&self, min: usize, max: usize) -> Result<(), Error>;
}

impl<T: HasUtf16CodeUnits> Utf16CodeUnits for T {
    fn validate_num_code_units(&self, min: usize, max: usize) -> Result<(), Error> {
        super::check_len(self.num_code_units(), min, max)
    }
}

impl<T: Utf16CodeUnits> Utf16CodeUnits for Option<T> {
    fn validate_num_code_units(&self, min: usize, max: usize) -> Result<(), Error> {
        match self {
            Some(v) => v.validate_num_code_units(min, max),
            None => Ok(()),
        }
    }
}

pub trait HasUtf16CodeUnits {
    fn num_code_units(&self) -> usize;
}

macro_rules! impl_str {
    ($(in<$lifetime:lifetime>)? $T:ty) => {
        impl<$($lifetime)?> HasUtf16CodeUnits for $T {
            fn num_code_units(&self) -> usize {
                self.encode_utf16().count()
            }
        }
    };
}

impl_str!(std::string::String);
impl_str!(in<'a> &'a std::string::String);
impl_str!(in<'a> &'a str);
impl_str!(in<'a> std::borrow::Cow<'a, str>);
impl_str!(std::rc::Rc<str>);
impl_str!(std::sync::Arc<str>);
impl_str!(std::boxed::Box<str>);
