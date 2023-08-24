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

pub fn default_ctx_name() -> syn::Ident {
    syn::Ident::new("__garde_user_ctx", proc_macro2::Span::call_site())
}
