use crate::error::Error;

pub fn validate<T: AsRef<str>>(field_name: &str, v: T, pat: &str) -> Result<(), Error> {
    todo!()
}
