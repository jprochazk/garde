use crate::error::Error;

pub fn apply<T: Alphanumeric>(v: &T, _: ()) -> Result<(), Error> {
    if !v.check_alphanumeric() {
        return Err(Error::new("not alphanumeric"));
    }
    Ok(())
}

#[cfg_attr(
    feature = "nightly-error-messages",
    rustc_on_unimplemented(
        message = "`{Self}` does not support alphanumeric validation",
        label = "This type does not support alphanumeric validation",
    )
)]
pub trait Alphanumeric {
    fn check_alphanumeric(&self) -> bool;
}

impl Alphanumeric for String {
    fn check_alphanumeric(&self) -> bool {
        self.chars().all(|c| c.is_alphanumeric())
    }
}
impl<'a> Alphanumeric for &'a str {
    fn check_alphanumeric(&self) -> bool {
        self.chars().all(|c| c.is_alphanumeric())
    }
}
impl<'a> Alphanumeric for std::borrow::Cow<'a, str> {
    fn check_alphanumeric(&self) -> bool {
        self.chars().all(|c| c.is_alphanumeric())
    }
}
