//! Error types used by `garde`.
//!
//! The entrypoint of this module is the [`Error`] type.
#![allow(dead_code)]

mod rc_list;
use std::borrow::Cow;

use compact_str::{CompactString, ToCompactString};
use smallvec::SmallVec;

use self::rc_list::List;

/// A validation error report.
///
/// This type is used as a container for errors aggregated during validation.
/// It is a flat list of `(Path, Error)`.
/// A single field or list item may have any number of errors attached to it.
///
/// It is possible to extract all errors for specific field using the [`select`][`crate::select`] macro.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Report {
    errors: Vec<(Path, Error)>,
}

impl Report {
    /// Create an empty [`Report`].
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    /// Append an [`Error`] into this report at the given [`Path`].
    pub fn append(&mut self, path: Path, error: Error) {
        self.errors.push((path, error));
    }

    /// Iterate over all `(Path, Error)` pairs.
    pub fn iter(&self) -> impl Iterator<Item = &(Path, Error)> {
        self.errors.iter()
    }

    /// Returns `true` if the report contains no validation errors.
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Converts into the inner validation errors.
    pub fn into_inner(self) -> Vec<(Path, Error)> {
        self.errors
    }
}

impl std::fmt::Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (path, error) in self.iter() {
            if path.is_empty() {
                writeln!(f, "{error}")?;
            } else {
                writeln!(f, "{path}: {error}")?;
            }
        }
        Ok(())
    }
}

impl std::error::Error for Report {}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Error {
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    code: Option<Cow<'static, str>>,
    message: CompactString,
}

impl Error {
    pub fn new(message: impl ToCompactString) -> Self {
        Self {
            code: None,
            message: message.to_compact_string(),
        }
    }

    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    pub fn message(&self) -> &str {
        self.message.as_ref()
    }

    pub fn set_code(&mut self, code: impl Into<Cow<'static, str>>) {
        self.code = Some(code.into());
    }

    pub fn with_code(mut self, code: impl Into<Cow<'static, str>>) -> Self {
        self.code = Some(code.into());
        self
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(code) = &self.code {
            write!(f, "[{}] ", code)?;
        }
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Path {
    components: List<(Kind, CompactString)>,
}

#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Kind {
    None,
    Key,
    Index,
}

/// Represents a path component without a key. This is useful when the container
/// only ever holds a single key, which is the case for any 1-tuple struct.
///
/// For an example usage, see the implementation of [Inner][`crate::rules::inner::Inner`] for `Option`.
#[derive(Default)]
pub struct NoKey(());

impl std::fmt::Display for NoKey {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}

pub trait PathComponentKind: std::fmt::Display + ToCompactString {
    fn component_kind() -> Kind;
}

macro_rules! impl_path_component_kind {
    ($(@$($G:lifetime)*;)? $T:ty => $which:ident) => {
        impl $(<$($G),*>)? PathComponentKind for $T {
            fn component_kind() -> Kind {
                Kind::$which
            }
        }
    }
}

impl_path_component_kind!(usize => Index);
impl_path_component_kind!(@'a; &'a str => Key);
impl_path_component_kind!(@'a; Cow<'a, str> => Key);
impl_path_component_kind!(String => Key);
impl_path_component_kind!(CompactString => Key);
impl_path_component_kind!(NoKey => None);

impl<T: PathComponentKind> PathComponentKind for &T {
    fn component_kind() -> Kind {
        T::component_kind()
    }
}

impl Path {
    pub fn empty() -> Self {
        Self {
            components: List::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.components.len()
    }

    pub fn is_empty(&self) -> bool {
        self.components.is_empty()
    }

    pub fn new<C: PathComponentKind>(component: C) -> Self {
        Self {
            components: List::new().append((C::component_kind(), component.to_compact_string())),
        }
    }

    pub fn join<C: PathComponentKind>(&self, component: C) -> Self {
        Self {
            components: self
                .components
                .append((C::component_kind(), component.to_compact_string())),
        }
    }

    #[doc(hidden)]
    pub fn __iter(
        &self,
    ) -> impl DoubleEndedIterator<Item = (Kind, &CompactString)> + ExactSizeIterator {
        let mut components = TempComponents::with_capacity(self.components.len());
        for (kind, component) in self.components.iter() {
            components.push((*kind, component));
        }
        components.into_iter()
    }
}

type TempComponents<'a> = SmallVec<[(Kind, &'a CompactString); 8]>;

impl std::fmt::Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct Components<'a> {
            path: &'a Path,
        }

        impl std::fmt::Debug for Components<'_> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut list = f.debug_list();
                list.entries(self.path.__iter().rev().map(|(_, c)| c))
                    .finish()
            }
        }

        f.debug_struct("Path")
            .field("components", &Components { path: self })
            .finish()
    }
}

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut components = self.__iter().rev().peekable();
        let mut first = true;
        while let Some((kind, component)) = components.next() {
            if first && kind == Kind::Index {
                f.write_str("[")?;
            }
            first = false;
            f.write_str(component.as_str())?;
            if kind == Kind::Index {
                f.write_str("]")?;
            }
            if let Some((kind, _)) = components.peek() {
                match kind {
                    Kind::None => {}
                    Kind::Key => f.write_str(".")?,
                    Kind::Index => f.write_str("[")?,
                }
            }
        }

        Ok(())
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for Path {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeSeq as _;

        let components = self.__iter().rev();
        let mut seq = serializer.serialize_seq(Some(components.len()))?;
        for component in components {
            seq.serialize_element(&component)?;
        }
        seq.end()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Path {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut components = List::new();
        for v in SmallVec::<[(Kind, CompactString); 8]>::deserialize(deserializer)? {
            components = components.append(v);
        }
        Ok(Path { components })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const _: () = {
        fn assert<T: Send>() {}
        let _ = assert::<Report>;
    };

    #[test]
    fn path_join() {
        let path = Path::new("a").join("b").join("c");
        assert_eq!(path.to_string(), "a.b.c");
    }

    #[test]
    fn report_select() {
        let mut report = Report::new();
        report.append(Path::new("a").join("b"), Error::new("lol"));
        report.append(
            Path::new("a").join("b").join("c"),
            Error::new("that seems wrong"),
        );
        report.append(Path::new("a").join("b").join("c"), Error::new("pog"));
        report.append(Path::new("array").join("0").join("c"), Error::new("pog"));

        assert_eq!(
            crate::select!(report, a.b.c).collect::<Vec<_>>(),
            [&Error::new("that seems wrong"), &Error::new("pog")]
        );

        assert_eq!(
            crate::select!(report, array[0].c).collect::<Vec<_>>(),
            [&Error::new("pog")]
        );
    }

    #[cfg(feature = "serde")]
    mod serde {
        use super::*;

        #[test]
        fn roundtrip_serde() {
            let mut report = Report::new();
            report.append(Path::new("a").join(0), Error::new("lorem"));
            report.append(Path::new("a").join(1), Error::new("ispum"));
            report.append(Path::new("a").join(2), Error::new("dolor"));
            report.append(Path::new("b").join("c"), Error::new("dolor"));

            let de: Report =
                serde_json::from_str(&serde_json::to_string(&report).unwrap()).unwrap();

            assert_eq!(report.errors, de.errors);
        }
    }
}
