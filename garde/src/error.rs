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
/// It is possible to extract all errors for specific field using the [`select`] macro.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
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
}

impl std::fmt::Display for Report {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (path, error) in self.iter() {
            writeln!(f, "{path}: {error}")?;
        }
        Ok(())
    }
}

impl std::error::Error for Report {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Error {
    message: CompactString,
}

impl Error {
    pub fn new(message: impl PathComponentKind) -> Self {
        Self {
            message: message.to_compact_string(),
        }
    }

    pub fn message(&self) -> &str {
        self.message.as_ref()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
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
pub enum Kind {
    Key,
    Index,
}

pub trait PathComponentKind: std::fmt::Display + ToCompactString + private::Sealed {
    fn component_kind() -> Kind;
}

macro_rules! impl_path_component_kind {
    ($(@$($G:lifetime)*;)? $T:ty => $which:ident) => {
        impl $(<$($G),*>)? private::Sealed for $T {}
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

impl<'a, T: PathComponentKind> private::Sealed for &'a T {}
impl<'a, T: PathComponentKind> PathComponentKind for &'a T {
    fn component_kind() -> Kind {
        T::component_kind()
    }
}

mod private {
    pub trait Sealed {}
}

impl Path {
    pub fn empty() -> Self {
        Self {
            components: List::new(),
        }
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
    pub fn __iter_components_rev(&self) -> rc_list::Iter<'_, (Kind, CompactString)> {
        self.components.iter()
    }

    #[doc(hidden)]
    pub fn __iter_components(&self) -> impl DoubleEndedIterator<Item = (Kind, &CompactString)> {
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

        impl<'a> std::fmt::Debug for Components<'a> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut list = f.debug_list();
                list.entries(self.path.__iter_components().rev().map(|(_, c)| c))
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
        let mut components = self.__iter_components().rev().peekable();
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
        let str = self.to_compact_string();
        serializer.serialize_str(str.as_str())
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! __select {
    ($report:expr, $($component:ident).*) => {{
        let report = &$report;
        let needle = [$(stringify!($component)),*];
        report.iter()
            .filter(move |(path, _)| {
                let components = path.__iter_components_rev();
                let needle = needle.iter().rev();

                components.map(|(_, v)| v.as_str()).zip(needle).all(|(a, b)| &a == b)
            })
            .map(|(_, error)| error)
    }}
}

pub use crate::__select as select;

#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(
            select!(report, a.b.c).collect::<Vec<_>>(),
            [&Error::new("that seems wrong"), &Error::new("pog")]
        );
    }
}
