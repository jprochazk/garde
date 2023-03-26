use crate::error::Error;

pub fn apply<T: Contains>(v: &T, (pat,): (&str,)) -> Result<(), Error> {
    if !v.check_contains(pat) {
        return Err(Error::new(format!("does not contain \"{pat}\"")));
    }
    Ok(())
}

#[cfg_attr(
    feature = "nightly-error-messages",
    rustc_on_unimplemented(
        message = "`{Self}` does not support substring validation",
        label = "This type does not support substring validation",
    )
)]
pub trait Contains {
    fn check_contains(&self, pat: &str) -> bool;
}

impl Contains for String {
    fn check_contains(&self, pat: &str) -> bool {
        self.contains(pat)
    }
}
impl<'a> Contains for &'a str {
    fn check_contains(&self, pat: &str) -> bool {
        self.contains(pat)
    }
}
impl<'a> Contains for std::borrow::Cow<'a, str> {
    fn check_contains(&self, pat: &str) -> bool {
        self.contains(pat)
    }
}
