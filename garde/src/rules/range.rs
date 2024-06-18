//! Range validation.
//!
//! ```rust
//! #[derive(garde::Validate)]
//! struct Test {
//!     #[garde(range(min=10,max=100))]
//!     v: u64,
//! }
//! ```
//!
//! The entrypoint is the [`Bounds`] trait. Implementing this trait for a type allows that type to be used with the `#[garde(range(...))]` rule.
//!
//! This trait is implemented for all primitive integer types.

use std::fmt::Display;

use crate::error::Error;

#[inline]
pub fn apply<T: Bounds>(
    v: &T,
    (min, max): (Option<T::Size>, Option<T::Size>),
) -> Result<(), Error> {
    let min = min.unwrap_or(T::MIN);
    let max = max.unwrap_or(T::MAX);
    if let Err(e) = v.validate_bounds(min, max) {
        match e {
            OutOfBounds::Lower => {
                return Err(Error::new("INVALID_SIZE", format!("lower than {min}")))
            }
            OutOfBounds::Upper => {
                return Err(Error::new("EXCEEDS_SIZE", format!("greater than {max}")))
            }
        }
    }
    Ok(())
}

pub trait Bounds: PartialOrd {
    type Size: Copy + Sized + Display;

    const MIN: Self::Size;
    const MAX: Self::Size;

    fn validate_bounds(
        &self,
        lower_bound: Self::Size,
        upper_bound: Self::Size,
    ) -> Result<(), OutOfBounds>;
}

pub enum OutOfBounds {
    Lower,
    Upper,
}

macro_rules! impl_for_int {
    ($($T:ident),*) => {
        $(
            impl Bounds for $T {
                type Size = $T;

                const MIN: Self::Size = $T::MIN;
                const MAX: Self::Size = $T::MAX;

                fn validate_bounds(
                    &self,
                    lower_bound: Self::Size,
                    upper_bound: Self::Size,
                ) -> Result<(), OutOfBounds> {
                    if self < &lower_bound {
                        Err(OutOfBounds::Lower)
                    } else if self > &upper_bound {
                        Err(OutOfBounds::Upper)
                    } else {
                        Ok(())
                    }
                }
            }
        )*
    };
}

impl_for_int!(u8, u16, u32, u64, usize, u128, i8, i16, i32, i64, isize, i128, f32, f64);

impl<T: Bounds> Bounds for Option<T> {
    type Size = T::Size;

    const MIN: Self::Size = T::MIN;
    const MAX: Self::Size = T::MAX;

    fn validate_bounds(
        &self,
        lower_bound: Self::Size,
        upper_bound: Self::Size,
    ) -> Result<(), OutOfBounds> {
        match self {
            Some(value) => value.validate_bounds(lower_bound, upper_bound),
            None => Ok(()),
        }
    }
}
