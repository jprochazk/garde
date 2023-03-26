use crate::error::Error;

#[cfg_attr(
    feature = "nightly-error-messages",
    rustc_on_unimplemented(
        message = "`{Self}` does not support prefix validation",
        label = "This type does not support prefix validation",
    )
)]
pub trait Prefix {
    fn has_prefix(&self, pat: &str) -> bool;
}

pub fn apply<T: Prefix>(v: &T, (pat,): (&str,)) -> Result<(), Error> {
    if !v.has_prefix(pat) {
        return Err(Error::new(format!("value does not begin with \"{pat}\"")));
    }
    Ok(())
}

impl Prefix for String {
    fn has_prefix(&self, pat: &str) -> bool {
        self.starts_with(pat)
    }
}
impl<'a> Prefix for &'a str {
    fn has_prefix(&self, pat: &str) -> bool {
        self.starts_with(pat)
    }
}
impl<'a> Prefix for std::borrow::Cow<'a, str> {
    fn has_prefix(&self, pat: &str) -> bool {
        self.starts_with(pat)
    }
}
