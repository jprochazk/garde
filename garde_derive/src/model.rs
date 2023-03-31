mod rules;

use std::collections::BTreeMap;

use proc_macro2::Ident;
use syn::{Expr, Generics, Type};

use self::rules::Rule;

pub enum Input {
    Struct(Struct),
    Enum(Enum),
}

pub struct Struct {
    pub ident: Ident,
    pub generics: Generics,
    pub context: Option<Type>,
    pub fields: BTreeMap<Ident, Field>,
}

pub struct Enum {
    pub variants: BTreeMap<Ident, Variant>,
}

pub struct Variant {
    pub name: Ident,
    pub kind: VariantKind,
}

pub enum VariantKind {
    Struct(StructVariant),
    Tuple(TupleVariant),
}

pub struct StructVariant {
    pub fields: BTreeMap<Ident, Field>,
}

pub struct TupleVariant {
    pub fields: Vec<Field>,
}

pub struct Field {
    pub name: Ident,
    pub alias: Option<Ident>,
    pub message: Option<Message>,
    pub code: Option<Ident>,
    pub validate: Validate,
}

pub enum Message {
    Fmt(String),
    Fn(Expr),
}

pub enum Validate {
    Skip,
    Rules(Vec<Rule>),
}
