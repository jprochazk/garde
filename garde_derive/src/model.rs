use std::collections::{BTreeMap, BTreeSet};

use proc_macro2::{Ident, Span};
use syn::{Expr, Generics, Path, Type};

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
    Transparent,
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
            Attr::Transparent => "transparent",
        }
    }
}

pub enum InputKind {
    Struct(Variant),
    Enum(Vec<(Ident, Option<Variant>)>),
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

// pub enum Message {
//     Fmt(Str),
//     Func(Expr),
// }

pub struct RawRule {
    pub span: Span,
    pub code: Option<Str>,
    pub kind: RawRuleKind,
}

pub enum RawRuleKind {
    Skip,
    Adapt(Path),
    Rename(Str),
    // Message(Message),
    // Code(Str),
    Dive(Option<Expr>),
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
    Length(RawLength),
    Matches(Path),
    Range(Range<Expr>),
    Contains(Expr),
    Prefix(Expr),
    Suffix(Expr),
    Pattern(Pattern),
    Custom(Expr),
    Inner(List<RawRule>),
}

pub struct RawLength {
    pub mode: LengthMode,
    pub range: Range<Either<usize, Expr>>,
}

#[derive(Clone, Copy, Default)]
pub enum LengthMode {
    #[default]
    Simple,
    Bytes,
    Chars,
    Graphemes,
    Utf16,
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
    pub equal: Option<T>,
}

pub struct List<T> {
    pub contents: Vec<T>,
}

pub struct Validate {
    pub ident: Ident,
    pub generics: Generics,
    pub context: (Type, Ident),
    pub is_transparent: bool,
    pub kind: ValidateKind,
    // I don't know why Rust thinks this is unused.
    // It's both read and written, grep for `.allow_unvalidated`.
    #[allow(dead_code)]
    pub options: Options,
}

pub struct Options {
    pub allow_unvalidated: bool,
}

pub enum ValidateKind {
    Struct(ValidateVariant),
    Enum(Vec<(Ident, Option<ValidateVariant>)>),
}

pub struct ValidateField {
    pub ty: Type,

    pub adapter: Option<Path>,
    pub skip: Option<Span>,
    pub alias: Option<String>,
    // pub message: Option<Message>,
    // pub code: Option<String>,
    pub dive: Option<(Span, Option<Expr>)>,
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
    Required {
        code: Option<String>,
    },
    Ascii {
        code: Option<String>,
    },
    Alphanumeric {
        code: Option<String>,
    },
    Email {
        code: Option<String>,
    },
    Url {
        code: Option<String>,
    },
    Ip {
        code: Option<String>,
    },
    IpV4 {
        code: Option<String>,
    },
    IpV6 {
        code: Option<String>,
    },
    CreditCard {
        code: Option<String>,
    },
    PhoneNumber {
        code: Option<String>,
    },
    LengthSimple {
        code: Option<String>,
        range: LengthRange,
    },
    LengthBytes {
        code: Option<String>,
        range: LengthRange,
    },
    LengthChars {
        code: Option<String>,
        range: LengthRange,
    },
    LengthGraphemes {
        code: Option<String>,
        range: LengthRange,
    },
    LengthUtf16 {
        code: Option<String>,
        range: LengthRange,
    },
    Matches {
        code: Option<String>,
        path: Path,
    },
    Range {
        code: Option<String>,
        range: ValidateRange<Expr>,
    },
    Contains {
        code: Option<String>,
        expr: Expr,
    },
    Prefix {
        code: Option<String>,
        expr: Expr,
    },
    Suffix {
        code: Option<String>,
        expr: Expr,
    },
    Pattern {
        code: Option<String>,
        pat: ValidatePattern,
    },
}

type LengthRange = ValidateRange<Either<usize, Expr>>;

impl ValidateRule {
    pub fn name(&self) -> &'static str {
        match self {
            ValidateRule::Required { .. } => "required",
            ValidateRule::Ascii { .. } => "ascii",
            ValidateRule::Alphanumeric { .. } => "alphanumeric",
            ValidateRule::Email { .. } => "email",
            ValidateRule::Url { .. } => "url",
            ValidateRule::Ip { .. } => "ip",
            ValidateRule::IpV4 { .. } => "ip",
            ValidateRule::IpV6 { .. } => "ip",
            ValidateRule::CreditCard { .. } => "credit_card",
            ValidateRule::PhoneNumber { .. } => "phone_number",
            ValidateRule::LengthSimple { .. } => "length::simple",
            ValidateRule::LengthBytes { .. } => "length::bytes",
            ValidateRule::LengthChars { .. } => "length::chars",
            ValidateRule::LengthGraphemes { .. } => "length::graphemes",
            ValidateRule::LengthUtf16 { .. } => "length::utf16",
            ValidateRule::Matches { .. } => "matches",
            ValidateRule::Range { .. } => "range",
            ValidateRule::Contains { .. } => "contains",
            ValidateRule::Prefix { .. } => "prefix",
            ValidateRule::Suffix { .. } => "suffix",
            ValidateRule::Pattern { .. } => "pattern",
        }
    }
}

pub enum ValidatePattern {
    #[cfg(feature = "regex")]
    Lit(String),
    Expr(Expr),
}

pub enum ValidateRange<T> {
    GreaterThan(T),
    LowerThan(T),
    Between(T, T),
    Equal(T),
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
