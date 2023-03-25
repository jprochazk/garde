use std::borrow::Cow;
use std::collections::BTreeMap;

/// This type encapsulates a single validation error.
#[derive(Clone, Debug)]
pub struct Error {
    pub message: Cow<'static, str>,
}

impl Error {
    pub fn new(message: Cow<'static, str>) -> Self {
        Self { message }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}

/// This type encapsulates a set of validation errors.
#[derive(Clone, Debug)]
pub struct Errors {
    pub fields: BTreeMap<&'static str, Error>,
}

impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut iter = self.fields.iter().peekable();
        while let Some((_, error)) = iter.next() {
            write!(f, "{error}")?;
            if iter.peek().is_some() {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl std::error::Error for Errors {}
