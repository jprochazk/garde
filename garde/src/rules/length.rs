use crate::error::Error;

pub fn validate<T: Length>(field_name: &str, v: T, min: usize, max: usize) -> Result<(), Error> {
    let length = v.len();
    if length < min {
        return Err(Error::new(
            format!("length of {field_name} is less than the required {min}").into(),
        ));
    }
    if length > max {
        return Err(Error::new(
            format!("length of {field_name} is greater than the required {max}").into(),
        ));
    }
    Ok(())
}

#[allow(clippy::len_without_is_empty)]
pub trait Length {
    fn len(&self) -> usize;
}
