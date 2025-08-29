//! URL validation using the [`url`] crate.
//!
//! ```rust
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(url)]
//!     v: String,
//! }
//! ```
//!
//! The entrypoint is the [`Url`] trait. Implementing this trait for a type allows that type to be used with the `#[garde(url)]` rule.
//!
//! The [`url`] crate only allows parsing from a `&str`, which is why this trait has a blanket implementation for all `T: garde::rules::AsStr`.
//!
//! If you need to implement this for a string-like type where a contiguous slice of the entire contents cannot be obtained,
//! then there is currently no way for you to implement this trait.

use super::AsStr;
use crate::error::Error;
pub use crate::i18n::InvalidUrl;

pub fn apply<T: Url>(v: &T, _: ()) -> Result<(), Error> {
    if let Err(e) = v.validate_url() {
        return Err(Error::new(i18n!(url_invalid, e)));
    }
    Ok(())
}

pub trait Url {
    fn validate_url(&self) -> Result<(), InvalidUrl>;
}

impl<T: AsStr> Url for T {
    fn validate_url(&self) -> Result<(), InvalidUrl> {
        url::Url::parse(self.as_str())
            .map(|_| ())
            .map_err(InvalidUrl::from)
    }
}

impl<T: Url> Url for Option<T> {
    fn validate_url(&self) -> Result<(), InvalidUrl> {
        match self {
            Some(value) => value.validate_url(),
            None => Ok(()),
        }
    }
}

impl From<url::ParseError> for InvalidUrl {
    fn from(e: url::ParseError) -> Self {
        match e {
            url::ParseError::EmptyHost => InvalidUrl::EmptyHost,
            url::ParseError::IdnaError => InvalidUrl::IdnaError,
            url::ParseError::InvalidPort => InvalidUrl::InvalidPort,
            url::ParseError::InvalidIpv4Address => InvalidUrl::InvalidIpv4Address,
            url::ParseError::InvalidIpv6Address => InvalidUrl::InvalidIpv6Address,
            url::ParseError::InvalidDomainCharacter => InvalidUrl::InvalidDomainCharacter,
            url::ParseError::RelativeUrlWithoutBase => InvalidUrl::RelativeUrlWithoutBase,
            url::ParseError::RelativeUrlWithCannotBeABaseBase => {
                InvalidUrl::RelativeUrlWithCannotBeABaseBase
            }
            url::ParseError::SetHostOnCannotBeABaseUrl => InvalidUrl::SetHostOnCannotBeABaseUrl,
            url::ParseError::Overflow => InvalidUrl::Overflow,
            _ => InvalidUrl::Other,
        }
    }
}

impl super::length::HasSimpleLength for url::Url {
    fn length(&self) -> usize {
        self.as_str().len()
    }
}

impl super::length::HasChars for url::Url {
    fn num_chars(&self) -> usize {
        self.as_str().chars().count()
    }
}

#[cfg(feature = "unicode")]
impl super::length::HasGraphemes for url::Url {
    fn num_graphemes(&self) -> usize {
        use unicode_segmentation::UnicodeSegmentation;

        self.as_str().graphemes(true).count()
    }
}

impl super::length::HasBytes for url::Url {
    fn num_bytes(&self) -> usize {
        self.as_str().len()
    }
}

impl super::length::HasUtf16CodeUnits for url::Url {
    fn num_code_units(&self) -> usize {
        self.as_str().encode_utf16().count()
    }
}
