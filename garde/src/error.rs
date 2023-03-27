use std::borrow::Cow;
use std::collections::BTreeMap;

/// This type encapsulates a single validation error.
#[derive(Clone, Debug)]
pub struct Error {
    pub message: Cow<'static, str>,
}

impl Error {
    pub fn new(message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}

/// This type encapsulates a set of (potentially nested) validation errors.
#[derive(Clone, Debug)]
pub enum Errors {
    Simple(Vec<Error>),
    List(Vec<Errors>),
    Fields(BTreeMap<Cow<'static, str>, Errors>),
}

impl From<Result<(), Errors>> for Errors {
    fn from(value: Result<(), Errors>) -> Self {
        match value {
            Ok(()) => Errors::empty(),
            Err(errors) => errors,
        }
    }
}

impl Errors {
    pub fn finish(self) -> Result<(), Errors> {
        if !self.is_empty() {
            Err(self)
        } else {
            Ok(())
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Errors::Simple(v) => v.is_empty(),
            Errors::List(v) => v.is_empty(),
            Errors::Fields(v) => v.is_empty(),
        }
    }

    pub fn flatten(self) -> Vec<(String, Error)> {
        fn flatten_inner(out: &mut Vec<(String, Error)>, current_path: String, errors: &Errors) {
            match errors {
                Errors::Simple(errors) => {
                    for error in errors {
                        out.push((current_path.clone(), error.clone()));
                    }
                }
                Errors::List(errors) => {
                    for (i, errors) in errors.iter().enumerate() {
                        flatten_inner(out, format!("{current_path}[{i}]"), errors);
                    }
                }
                Errors::Fields(errors) => {
                    for (key, errors) in errors.iter() {
                        flatten_inner(out, format!("{current_path}.{key}"), errors);
                    }
                }
            }
        }

        let mut errors = vec![];
        flatten_inner(&mut errors, "value".to_string(), &self);
        errors
    }

    pub fn empty() -> Self {
        Errors::Simple(Vec::new())
    }

    pub fn simple<F>(f: F) -> Errors
    where
        F: FnMut(&mut SimpleErrorBuilder),
    {
        SimpleErrorBuilder::dive(f)
    }

    pub fn list<F>(f: F) -> Errors
    where
        F: FnMut(&mut ListErrorBuilder),
    {
        ListErrorBuilder::dive(f)
    }

    pub fn fields<F>(f: F) -> Errors
    where
        F: FnMut(&mut FieldsErrorBuilder),
    {
        FieldsErrorBuilder::dive(f)
    }
}

// TODO: remove rename, change rules to not require field_name

#[doc(hidden)]
pub struct SimpleErrorBuilder {
    inner: Vec<Error>,
}

impl SimpleErrorBuilder {
    fn dive<F>(mut f: F) -> Errors
    where
        F: FnMut(&mut SimpleErrorBuilder),
    {
        let mut builder = SimpleErrorBuilder { inner: Vec::new() };
        f(&mut builder);
        Errors::Simple(builder.inner)
    }

    pub fn push(&mut self, error: Error) {
        self.inner.push(error);
    }
}

#[doc(hidden)]
pub struct ListErrorBuilder {
    inner: Vec<Errors>,
}

impl ListErrorBuilder {
    fn dive<F>(mut f: F) -> Errors
    where
        F: FnMut(&mut ListErrorBuilder),
    {
        let mut builder = ListErrorBuilder { inner: Vec::new() };
        f(&mut builder);
        Errors::List(builder.inner)
    }

    pub fn push(&mut self, entry: impl Into<Errors>) {
        let entry = entry.into();

        if entry.is_empty() {
            return;
        }

        self.inner.push(entry);
    }
}

#[doc(hidden)]
pub struct FieldsErrorBuilder {
    inner: BTreeMap<Cow<'static, str>, Errors>,
}

impl FieldsErrorBuilder {
    fn dive<F>(mut f: F) -> Errors
    where
        F: FnMut(&mut FieldsErrorBuilder),
    {
        let mut builder = FieldsErrorBuilder {
            inner: BTreeMap::new(),
        };
        f(&mut builder);
        Errors::Fields(builder.inner)
    }

    pub fn insert(&mut self, field: impl Into<Cow<'static, str>>, entry: impl Into<Errors>) {
        let entry = entry.into();

        if entry.is_empty() {
            return;
        }

        let existing = self.inner.insert(field.into(), entry);
        assert!(
            existing.is_none(),
            "each field should only be dived into once"
        )
    }
}

impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Errors::Simple(v) => {
                let mut iter = v.iter().peekable();
                while let Some(error) = iter.next() {
                    write!(f, "{error}")?;
                    if iter.peek().is_some() {
                        writeln!(f)?;
                    }
                }
            }
            Errors::List(v) => {
                let mut iter = v.iter().peekable();
                while let Some(error) = iter.next() {
                    write!(f, "{error}")?;
                    if iter.peek().is_some() {
                        writeln!(f)?;
                    }
                }
            }
            Errors::Fields(v) => {
                let mut iter = v.values().peekable();
                while let Some(error) = iter.next() {
                    write!(f, "{error}")?;
                    if iter.peek().is_some() {
                        writeln!(f)?;
                    }
                }
            }
        };
        Ok(())
    }
}

impl std::error::Error for Errors {}
