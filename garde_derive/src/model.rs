use std::collections::{BTreeMap, BTreeSet};

use proc_macro2::{Ident, Span};
use syn::{Expr, ExprClosure, ExprPath, Generics, Type};

pub struct Input {
    pub ident: Ident,
    pub generics: Generics,
    pub attrs: Vec<(Span, Attr)>,
    pub kind: InputKind,
}

#[repr(u8)]
pub enum Attr {
    Context(Box<Type>, Ident),
    AllowUnvalidated,
}

impl Attr {
    pub fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)`
        // `union` between `repr(C)` structs, each of which has the `u8`
        // discriminant as its first field, so we can read the discriminant
        // without offsetting the pointer.
        unsafe { <*const _>::from(self).cast::<u8>().read() }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Attr::Context(..) => "context",
            Attr::AllowUnvalidated => "allow_unvalidated",
        }
    }
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
    Required,
    Ascii,
    Alphanumeric,
    Email,
    Url,
    Ip,
    IpV4,
    IpV6,
    CreditCard,
    PhoneNumber,
    Length(Range<Either<usize, Expr>>),
    ByteLength(Range<Either<usize, Expr>>),
    Range(Range<Expr>),
    Contains(Expr),
    Prefix(Expr),
    Suffix(Expr),
    Pattern(Pattern),
    Custom(Func),
    Inner(List<RawRule>),
}

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R> quote::ToTokens for Either<L, R>
where
    L: quote::ToTokens,
    R: quote::ToTokens,
{
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Self::Left(left) => left.to_tokens(tokens),
            Self::Right(right) => right.to_tokens(tokens),
        }
    }
}

pub enum Pattern {
    Lit(Str),
    Expr(Expr),
}

pub struct Str {
    pub span: Span,
    pub value: String,
}

pub struct Range<T> {
    pub span: Span,
    pub min: Option<T>,
    pub max: Option<T>,
}

pub struct List<T> {
    pub span: Span,
    pub contents: Vec<T>,
}

pub struct Validate {
    pub ident: Ident,
    pub generics: Generics,
    pub context: (Type, Ident),
    pub kind: ValidateKind,
    pub options: Options,
}

pub struct Options {
    pub allow_unvalidated: bool,
}

pub enum ValidateKind {
    Struct(ValidateVariant),
    Enum(Vec<(Ident, ValidateVariant)>),
}

pub struct ValidateField {
    pub ty: Type,

    pub skip: Option<Span>,
    pub alias: Option<String>,
    pub message: Option<Message>,
    pub code: Option<String>,

    pub dive: Option<Span>,
    pub rule_set: RuleSet,
}

impl ValidateField {
    pub fn is_empty(&self) -> bool {
        self.dive.is_none() && self.rule_set.is_empty()
    }

    pub fn has_top_level_rules(&self) -> bool {
        self.rule_set.has_top_level_rules()
    }
}

pub struct RuleSet {
    pub rules: BTreeSet<ValidateRule>,
    pub custom_rules: Vec<Expr>,
    pub inner: Option<Box<RuleSet>>,
}

impl RuleSet {
    pub fn empty() -> Self {
        Self {
            rules: BTreeSet::new(),
            custom_rules: Vec::new(),
            inner: None,
        }
    }

    pub fn is_empty(&self) -> bool {
        let inner_empty = match &self.inner {
            Some(inner) => inner.is_empty(),
            None => true,
        };
        inner_empty && self.rules.is_empty() && self.custom_rules.is_empty()
    }

    pub fn has_top_level_rules(&self) -> bool {
        !self.rules.is_empty() || !self.custom_rules.is_empty()
    }
}

#[repr(u8)]
pub enum ValidateRule {
    Required,
    Ascii,
    Alphanumeric,
    Email,
    Url,
    Ip,
    IpV4,
    IpV6,
    CreditCard,
    PhoneNumber,
    Length(ValidateRange<Either<usize, Expr>>),
    ByteLength(ValidateRange<Either<usize, Expr>>),
    Range(ValidateRange<Expr>),
    Contains(Expr),
    Prefix(Expr),
    Suffix(Expr),
    Pattern(ValidatePattern),
}

impl ValidateRule {
    pub fn name(&self) -> &'static str {
        match self {
            ValidateRule::Required => "required",
            ValidateRule::Ascii => "ascii",
            ValidateRule::Alphanumeric => "alphanumeric",
            ValidateRule::Email => "email",
            ValidateRule::Url => "url",
            ValidateRule::Ip => "ip",
            ValidateRule::IpV4 => "ip",
            ValidateRule::IpV6 => "ip",
            ValidateRule::CreditCard => "credit_card",
            ValidateRule::PhoneNumber => "phone_number",
            ValidateRule::Length { .. } => "length",
            ValidateRule::ByteLength { .. } => "byte_length",
            ValidateRule::Range { .. } => "range",
            ValidateRule::Contains(_) => "contains",
            ValidateRule::Prefix(_) => "prefix",
            ValidateRule::Suffix(_) => "suffix",
            ValidateRule::Pattern(_) => "pattern",
        }
    }
}

pub enum ValidatePattern {
    Lit(String),
    Expr(Expr),
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

impl PartialEq for ValidateRule {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Eq for ValidateRule {}

impl ValidateRule {
    fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)`
        // `union` between `repr(C)` structs, each of which has the `u8`
        // discriminant as its first field, so we can read the discriminant
        // without offsetting the pointer.
        unsafe { <*const _>::from(self).cast::<u8>().read() }
    }
}

impl PartialOrd for ValidateRule {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ValidateRule {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // `ValidateRuleKind` is intentionally only compared by the discriminant,
        // because we want there to only be one of each kind, without caring about
        // the value.
        self.discriminant().cmp(&other.discriminant())
    }
}
