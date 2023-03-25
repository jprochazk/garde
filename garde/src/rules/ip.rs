use crate::error::Error;

pub fn validate<T: AsRef<str>>(field_name: &str, v: T) -> Result<(), Error> {
    use std::str::FromStr;
    let v = v.as_ref();
    if let Err(e) = std::net::IpAddr::from_str(v) {
        return Err(Error::new(
            format!("{field_name} is not a valid ip address, {e}").into(),
        ));
    }
    Ok(())
}
