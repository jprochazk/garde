use crate::error::Error;

pub fn apply<T: Pattern>(v: &T, (pat,): (&regex::Regex,)) -> Result<(), Error> {
    if !v.matches(pat) {
        return Err(Error::new(format!("does not match pattern /{pat}/")));
    }
    Ok(())
}

#[cfg_attr(
    feature = "nightly-error-messages",
    rustc_on_unimplemented(
        message = "`{Self}` does not support pattern validation",
        label = "This type does not support pattern validation",
    )
)]
pub trait Pattern {
    fn matches(&self, pat: &Regex) -> bool;
}

#[doc(hidden)]
pub type StaticPattern = once_cell::sync::Lazy<Regex>;

#[doc(hidden)]
pub use regex::Regex;

#[doc(hidden)]
#[macro_export]
macro_rules! __init_pattern {
    ($pat:literal) => {
        $crate::rules::pattern::StaticPattern::new(|| {
            $crate::rules::pattern::Regex::new($pat).unwrap()
        })
    };
}
#[doc(hidden)]
pub use crate::__init_pattern as init_pattern;

fn check_str(v: &str, pat: &Regex) -> bool {
    pat.is_match(v)
}

impl Pattern for String {
    fn matches(&self, pat: &Regex) -> bool {
        check_str(self.as_str(), pat)
    }
}

impl<'a> Pattern for &'a str {
    fn matches(&self, pat: &Regex) -> bool {
        check_str(self, pat)
    }
}

impl<'a> Pattern for std::borrow::Cow<'a, str> {
    fn matches(&self, pat: &Regex) -> bool {
        check_str(self.as_ref(), pat)
    }
}
