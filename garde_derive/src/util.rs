pub trait MaybeFoldError {
    fn maybe_fold(&mut self, e: syn::Error);
}

impl MaybeFoldError for Option<syn::Error> {
    fn maybe_fold(&mut self, e: syn::Error) {
        match self {
            Some(v) => {
                v.combine(e);
            }
            None => *self = Some(e),
        }
    }
}
