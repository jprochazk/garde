use crate::error::Error;

pub fn apply<T: Bounds>(field_name: &str, v: &T, min: &T, max: &T) -> Result<(), Error> {
    if let Err(e) = v.check_bounds(min, max) {
        match e {
            OutOfBounds::Lower => {
                return Err(Error::new(
                    format!("`{field_name}` is out of bounds").into(),
                ))
            }
            OutOfBounds::Upper => {
                return Err(Error::new(
                    format!("`{field_name}` is out of bounds").into(),
                ))
            }
        }
    }
    Ok(())
}

#[cfg_attr(
    feature = "nightly-error-messages",
    rustc_on_unimplemented(
        message = "`{Self}` does not support bounds validation",
        label = "This type does not support bounds validation",
        note = "try implementing `garde::rules::range::Bounds` for `{Self}`"
    )
)]
pub trait Bounds: PartialOrd {
    const MIN: Self;
    const MAX: Self;
    fn check_bounds(&self, lower_bound: &Self, upper_bound: &Self) -> Result<(), OutOfBounds>;
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

                fn check_bounds(
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
