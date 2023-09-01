//! Error types used by `garde`.
//!
//! The entrypoint of this module is the [`Error`] type.
#![allow(dead_code)]

mod rc_list;
use std::borrow::Cow;

use compact_str::{CompactString, ToCompactString};
use smallvec::SmallVec;

use self::rc_list::List;

#[derive(Debug)]
pub struct Report {
    errors: Vec<(Path, Error)>,
}

impl Report {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn append(&mut self, path: Path, error: Error) {
        self.errors.push((path, error));
    }

    pub fn iter(&self) -> impl Iterator<Item = &(Path, Error)> {
        self.errors.iter()
    }

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

pub trait PathComponentKind: ToCompactString + private::Sealed {
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
                let mut components = TempComponents::with_capacity(self.path.components.len());

                for (kind, component) in self.path.components.iter() {
                    components.push((*kind, component));
                }
                list.entries(components.iter().rev().map(|(_, c)| c))
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
        let mut components = TempComponents::with_capacity(self.components.len());
        for (kind, component) in self.components.iter() {
            components.push((*kind, component));
        }

        let mut components = components.iter().rev().peekable();
        let mut first = true;
        while let Some((kind, component)) = components.next() {
            if first && kind == &Kind::Index {
                f.write_str("[")?;
            }
            first = false;
            f.write_str(component.as_str())?;
            if kind == &Kind::Index {
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
