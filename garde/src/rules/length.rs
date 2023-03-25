use crate::error::Error;

pub fn apply<T: Length>(field_name: &str, v: &T, min: usize, max: usize) -> Result<(), Error> {
    if let Err(e) = v.check_length(min, max) {
        match e {
            InvalidLength::Min => {
                return Err(Error::new(
                    format!("length of `{field_name}` is less than {min}").into(),
                ))
            }
            InvalidLength::Max => {
                return Err(Error::new(
                    format!("length of `{field_name}` is greater than {max}").into(),
                ))
            }
        }
    }
    Ok(())
}

#[cfg_attr(
    feature = "nightly-error-messages",
    rustc_on_unimplemented(
        message = "`{Self}` does not support length validation",
        label = "This type does not support length validation",
        note = "try implementing `garde::rules::length::Size` for `{Self}`"
    )
)]
pub trait Length {
    fn check_length(&self, min: usize, max: usize) -> Result<(), InvalidLength>;
}

pub enum InvalidLength {
    Min,
    Max,
}

#[allow(clippy::len_without_is_empty)]
pub trait Size {
    fn size(&self) -> usize;
}

impl<T: Size> Length for T {
    fn check_length(&self, min: usize, max: usize) -> Result<(), InvalidLength> {
        let len = self.size();
        if len < min {
            Err(InvalidLength::Min)
        } else if len > max {
            Err(InvalidLength::Max)
        } else {
            Ok(())
        }
    }
}

impl Size for String {
    fn size(&self) -> usize {
        self.chars().count()
    }
}

impl<'a> Size for &'a String {
    fn size(&self) -> usize {
        self.chars().count()
    }
}

impl<'a> Size for &'a str {
    fn size(&self) -> usize {
        self.chars().count()
    }
}

impl<'a> Size for std::borrow::Cow<'a, str> {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<T> Size for Vec<T> {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<'a, T> Size for &'a Vec<T> {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<T> Size for &[T] {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<T, const N: usize> Size for [T; N] {
    fn size(&self) -> usize {
        N
    }
}

impl<T, const N: usize> Size for &[T; N] {
    fn size(&self) -> usize {
        N
    }
}

impl<'a, K, V, S> Size for &'a std::collections::HashMap<K, V, S> {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<K, V, S> Size for std::collections::HashMap<K, V, S> {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<'a, T, S> Size for &'a std::collections::HashSet<T, S> {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<T, S> Size for std::collections::HashSet<T, S> {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<'a, K, V> Size for &'a std::collections::BTreeMap<K, V> {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<K, V> Size for std::collections::BTreeMap<K, V> {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<'a, T> Size for &'a std::collections::BTreeSet<T> {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<T> Size for std::collections::BTreeSet<T> {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<T> Size for std::collections::VecDeque<T> {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<T> Size for std::collections::BinaryHeap<T> {
    fn size(&self) -> usize {
        self.len()
    }
}

impl<T> Size for std::collections::LinkedList<T> {
    fn size(&self) -> usize {
        self.len()
    }
}
