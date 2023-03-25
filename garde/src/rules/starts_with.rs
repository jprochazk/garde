use crate::error::Error;

pub fn validate<T: AsRef<str>>(field_name: &str, v: T, pat: &str) -> Result<(), Error> {
    let v = v.as_ref();
    if !v.starts_with(pat) {
        return Err(Error::new(
            format!("{field_name} must start with {pat}").into(),
        ));
    }
    Ok(())
}
