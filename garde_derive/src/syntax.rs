use syn::DeriveInput;

use crate::model::Input;

pub enum Error {}

pub fn parse(input: DeriveInput) -> Result<Input, Error> {
    todo!()
}
