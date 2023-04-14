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

pub fn apply<T: Bounds>(v: &T, (min, max): (&T, &T)) -> Result<(), Error> {
    if let Err(e) = v.validate_bounds(min, max) {
        match e {
            OutOfBounds::Lower => return Err(Error::new(format!("lower than {min}"))),
            OutOfBounds::Upper => return Err(Error::new(format!("greater than {max}"))),
        }
    }
    Ok(())
}

pub trait Bounds: PartialOrd + Display {
    const MIN: Self;
    const MAX: Self;

    fn validate_bounds(&self, lower_bound: &Self, upper_bound: &Self) -> Result<(), OutOfBounds>;
}

pub enum OutOfBounds {
    Lower,
    Upper,
}

macro_rules! impl_for_int {
    ($($T:ident),*) => {
        $(
            impl Bounds for $T {
                const MIN: Self = $T::MIN;
                const MAX: Self = $T::MAX;

                fn validate_bounds(
                    &self,
                    lower_bound: &Self,
                    upper_bound: &Self,
                ) -> Result<(), OutOfBounds> {
                    if self < lower_bound {
                        Err(OutOfBounds::Lower)
                    } else if self > upper_bound {
                        Err(OutOfBounds::Upper)
                    } else {
                        Ok(())
                    }
                }
            }
        )*
    };
}

impl_for_int!(u8, u16, u32, u64, usize, u128, i8, i16, i32, i64, isize, i128);
