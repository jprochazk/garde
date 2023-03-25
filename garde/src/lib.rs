pub mod error;
pub mod rules;
pub mod validate;

pub use error::Errors;
pub use validate::Validate;

/*
TODO: Mention `validator` crate in README as prior art, maybe there are others?
TODO: obscure error messages? maybe in web framework specific crates
*/
