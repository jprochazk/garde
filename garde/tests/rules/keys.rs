use std::collections::HashMap;

use super::util;

#[derive(Debug, garde::Validate)]
struct Key<'a> {
    #[garde(inner(length(min = 1)), keys(length(min = 1)))]
    inner: HashMap<&'a str, &'a str>,
}
