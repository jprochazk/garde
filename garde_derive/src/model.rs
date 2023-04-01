use std::collections::{BTreeMap, BTreeSet};

use proc_macro2::{Ident, Span};
use syn::{Expr, ExprClosure, ExprPath, Generics, Type};

pub struct Input {
    pub ident: Ident,
    pub generics: Generics,
    pub attrs: Vec<(Span, Attr)>,
    pub kind: InputKind,
}

pub enum Attr {
    Context(Box<Type>),
}

pub enum InputKind {
    Struct(Variant),
    Enum(Vec<(Ident, Variant)>),
}

impl InputKind {
    pub fn empty() -> Self {
        Self::Struct(Variant::Tuple(Vec::new()))
    }
}

pub enum Variant {
    Struct(BTreeMap<Ident, Field>),
    Tuple(Vec<Field>),
}

impl Variant {
    pub fn empty() -> Self {
        Self::Tuple(Vec::new())
    }
}

pub struct Field {
    pub ty: Type,
    pub rules: Vec<RawRule>,
}

pub enum Func {
    Closure(ExprClosure),
    Path(ExprPath),
}

impl Func {
    pub fn expr(self) -> syn::Expr {
        match self {
            Func::Closure(v) => syn::Expr::Closure(v),
            Func::Path(v) => syn::Expr::Path(v),
        }
    }
}

pub enum Message {
    Fmt(Str),
    Func(Func),
}

pub struct RawRule {
    pub span: Span,
    pub kind: RawRuleKind,
}

pub enum RawRuleKind {
    Skip,
    Rename(Str),
    Message(Message),
    Code(Str),
    Dive,
    Ascii,
    Alphanumeric,
    Email,
    Url,
    Ip,
    IpV4,
    IpV6,
    CreditCard,
    PhoneNumber,
    Length(Range<usize>),
    ByteLength(Range<usize>),
    Range(Range<Expr>),
    Contains(Str),
    Prefix(Str),
    Suffix(Str),
    Pattern(Str),
    Custom(Func),
    Inner(Box<RawRule>),
}

pub struct Str {
    pub value: String,
}

pub struct Range<T> {
    pub span: Span,
    pub min: Option<T>,
    pub max: Option<T>,
}

pub struct Validate {
    pub ident: Ident,
    pub generics: Generics,
    pub context: Type,
    pub kind: ValidateKind,
}

pub enum ValidateKind {
    Struct(ValidateVariant),
    Enum(Vec<(Ident, ValidateVariant)>),
}

pub struct ValidateField {
    pub ty: Type,

    pub skip: Skip,
    pub alias: Option<String>,
    pub message: Option<Message>,
    pub code: Option<String>,

    pub dive: bool,
    pub rules: BTreeSet<ValidateRule>,
    pub custom_rules: Vec<Expr>,
}

pub struct Skip {
    pub span: Span,
    pub value: bool,
}

impl ValidateField {
    pub fn is_empty(&self) -> bool {
        !self.dive && self.rules.is_empty() && self.custom_rules.is_empty()
    }

    pub fn has_top_level_rules(&self) -> bool {
        !self.rules.is_empty() || !self.custom_rules.is_empty()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct ValidateRule {
    pub kind: ValidateRuleKind,
    pub depth: usize,
}

#[repr(u8)]
pub enum ValidateRuleKind {
    Ascii,
    Alphanumeric,
    Email,
    Url,
    Ip,
    IpV4,
    IpV6,
    CreditCard,
    PhoneNumber,
    Length(ValidateRange<usize>),
    ByteLength(ValidateRange<usize>),
    Range(ValidateRange<Expr>),
    Contains(String),
    Prefix(String),
    Suffix(String),
    Pattern(String),
}

impl ValidateRule {
    pub fn name(&self) -> &'static str {
        match &self.kind {
            ValidateRuleKind::Ascii => "ascii",
            ValidateRuleKind::Alphanumeric => "alphanumeric",
            ValidateRuleKind::Email => "email",
            ValidateRuleKind::Url => "url",
            ValidateRuleKind::Ip => "ip",
            ValidateRuleKind::IpV4 => "ip",
            ValidateRuleKind::IpV6 => "ip",
            ValidateRuleKind::CreditCard => "credit_card",
            ValidateRuleKind::PhoneNumber => "phone_number",
            ValidateRuleKind::Length { .. } => "length",
            ValidateRuleKind::ByteLength { .. } => "byte_length",
            ValidateRuleKind::Range { .. } => "range",
            ValidateRuleKind::Contains(_) => "contains",
            ValidateRuleKind::Prefix(_) => "prefix",
            ValidateRuleKind::Suffix(_) => "suffix",
            ValidateRuleKind::Pattern(_) => "pattern",
        }
    }
}

pub enum ValidateRange<T> {
    GreaterThan(T),
    LowerThan(T),
    Between(T, T),
}

pub enum ValidateVariant {
    Struct(Vec<(Ident, ValidateField)>),
    Tuple(Vec<ValidateField>),
}

impl ValidateVariant {
    pub fn empty() -> Self {
        Self::Tuple(Vec::new())
    }
}

impl PartialEq for ValidateRuleKind {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Eq for ValidateRuleKind {}

impl ValidateRuleKind {
    fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)`
        // `union` between `repr(C)` structs, each of which has the `u8`
        // discriminant as its first field, so we can read the discriminant
        // without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

impl PartialOrd for ValidateRuleKind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.discriminant().partial_cmp(&other.discriminant())
    }
}

impl Ord for ValidateRuleKind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.discriminant().cmp(&other.discriminant())
    }
}
