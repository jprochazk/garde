pub trait MaybeFoldError {
    fn maybe_fold(&mut self, error: syn::Error);
}

impl MaybeFoldError for Option<syn::Error> {
    fn maybe_fold(&mut self, error: syn::Error) {
        match self {
            Some(v) => {
                v.combine(error);
            }
            None => *self = Some(error),
        }
    }
}
