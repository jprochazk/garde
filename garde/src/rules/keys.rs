//! Key validation.
//!
//! ```rust
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(keys(length(min=1)))]
//!     v: HashMap<String, String>,
//! }
//! ```
//!
//! The entrypoint is the [`Keys`] trait. Implementing this trait for a type allows that type to be used with the `#[garde(keys(..))]` rule.

use crate::error::PathComponentKind;

pub fn apply<T, K, F>(field: &T, f: F)
where
    T: Keys<K>,
    K: PathComponentKind,
    F: FnMut(&K),
{
    field.validate_keys(f)
}

pub trait Keys<K: PathComponentKind> {
    fn validate_keys<F>(&self, f: F)
    where
        F: FnMut(&K);
}

impl<K, V> Keys<K> for std::collections::HashMap<K, V>
where
    K: PathComponentKind,
{
    fn validate_keys<F>(&self, mut f: F)
    where
        F: FnMut(&K),
    {
        for key in self.keys() {
            f(key)
        }
    }
}

impl<K, V> Keys<K> for std::collections::BTreeMap<K, V>
where
    K: PathComponentKind,
{
    fn validate_keys<F>(&self, mut f: F)
    where
        F: FnMut(&K),
    {
        for key in self.keys() {
            f(key)
        }
    }
}
