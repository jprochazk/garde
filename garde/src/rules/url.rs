use std::fmt::Display;

use crate::error::Error;

pub fn apply<T: Url>(v: &T, _: ()) -> Result<(), Error> {
    if let Err(e) = v.try_parse_url() {
        return Err(Error::new(format!("not a valid url: {e}")));
    }
    Ok(())
}

#[cfg_attr(
    feature = "nightly-error-messages",
    rustc_on_unimplemented(
        message = "`{Self}` does not support URL validation",
        label = "This type does not support URL validation",
    )
)]
pub trait Url {
    type Error: Display;
    fn try_parse_url(&self) -> Result<(), Self::Error>;
}

impl Url for String {
    type Error = url::ParseError;

    fn try_parse_url(&self) -> Result<(), Self::Error> {
        let _ = url::Url::parse(self.as_str())?;
        Ok(())
    }
}
impl<'a> Url for &'a str {
    type Error = url::ParseError;

    fn try_parse_url(&self) -> Result<(), Self::Error> {
        let _ = url::Url::parse(self)?;
        Ok(())
    }
}
impl<'a> Url for std::borrow::Cow<'a, str> {
    type Error = url::ParseError;

    fn try_parse_url(&self) -> Result<(), Self::Error> {
        let _ = url::Url::parse(self.as_ref())?;
        Ok(())
    }
}
