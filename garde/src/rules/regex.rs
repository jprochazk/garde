use crate::error::Error;

pub fn validate<T: AsRef<str>>(field_name: &str, v: T, regex: &regex::Regex) -> Result<(), Error> {
    let v = v.as_ref();
    if !regex.is_match(v) {
        return Err(Error::new(
            format!("{field_name} is not valid according to the regular expression /{regex}/")
                .into(),
        ));
    }
    Ok(())
}
