use crate::error::Error;

pub fn apply<T: Ascii>(field_name: &str, v: &T) -> Result<(), Error> {
    if !v.check_ascii() {
        return Err(Error::new(format!("`{field_name}` is not ascii").into()));
    }
    Ok(())
}

#[cfg_attr(
    feature = "nightly-error-messages",
    rustc_on_unimplemented(
        message = "`{Self}` does not support ascii validation",
        label = "This type does not support ascii validation",
    )
)]
pub trait Ascii {
    fn check_ascii(&self) -> bool;
}

impl Ascii for String {
    fn check_ascii(&self) -> bool {
        self.is_ascii()
    }
}
impl<'a> Ascii for &'a str {
    fn check_ascii(&self) -> bool {
        self.is_ascii()
    }
}
impl<'a> Ascii for std::borrow::Cow<'a, str> {
    fn check_ascii(&self) -> bool {
        self.is_ascii()
    }
}
