use crate::error::Error;

pub fn validate<T: AsRef<str>>(field_name: &str, v: T) -> Result<(), Error> {
    use std::str::FromStr;
    let v = v.as_ref();
    let v = match phonenumber::PhoneNumber::from_str(v) {
        Ok(v) => v,
        Err(e) => {
            return Err(Error::new(
                format!("{field_name} is not a valid phone number: {e}").into(),
            ))
        }
    };
    if !v.is_valid() {
        return Err(Error::new(
            format!("{field_name} is not a valid phone number").into(),
        ));
    }
    Ok(())
}
