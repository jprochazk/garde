mod check;
mod emit;
mod model;
mod syntax;
mod util;

use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(Validate, attributes(garde))]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let input = match syntax::parse(input) {
        Ok(v) => v,
        Err(e) => return e.into_compile_error().into(),
    };
    let input = match check::check(input) {
        Ok(v) => v,
        Err(e) => return e.into_compile_error().into(),
    };
    emit::emit(input).into()
}
