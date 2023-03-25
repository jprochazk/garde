use crate::error::Error;

pub fn validate<T: AsRef<str>>(field_name: &str, v: T) -> Result<(), Error> {
    let v = v.as_ref();
    if let Err(e) = url::Url::parse(v) {
        return Err(Error::new(
            format!("{field_name} is not a valid url, {e}").into(),
        ));
    }
    Ok(())
}
