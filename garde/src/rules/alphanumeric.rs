use crate::error::Error;

// TODO: `AsRef<str>` is too restrictive, you may have a string-like type, but without it being contiguous in memory (such as https://docs.rs/ropey/latest/ropey/)

pub fn validate<T: AsRef<str>>(field_name: &str, v: T) -> Result<(), Error> {
    todo!()
}
