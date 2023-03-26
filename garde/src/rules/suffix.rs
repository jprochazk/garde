use crate::error::Error;

#[cfg_attr(
    feature = "nightly-error-messages",
    rustc_on_unimplemented(
        message = "`{Self}` does not support suffix validation",
        label = "This type does not support suffix validation",
    )
)]
pub trait Suffix {
    fn has_suffix(&self, pat: &str) -> bool;
}

pub fn apply<T: Suffix>(v: &T, (pat,): (&str,)) -> Result<(), Error> {
    if !v.has_suffix(pat) {
        return Err(Error::new(format!("does not end with \"{pat}\"")));
    }
    Ok(())
}

impl Suffix for String {
    fn has_suffix(&self, pat: &str) -> bool {
        self.ends_with(pat)
    }
}
impl<'a> Suffix for &'a str {
    fn has_suffix(&self, pat: &str) -> bool {
        self.ends_with(pat)
    }
}
impl<'a> Suffix for std::borrow::Cow<'a, str> {
    fn has_suffix(&self, pat: &str) -> bool {
        self.ends_with(pat)
    }
}
