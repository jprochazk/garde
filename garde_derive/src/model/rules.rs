use syn::{Expr, ExprClosure, Path};

pub enum Rule {
    Ascii,
    Alphanumeric,
    Email,
    Url,
    Ip,
    IpV4,
    IpV6,
    CreditCard,
    PhoneNumber,
    Length(Box<Range<usize>>),
    ByteLength(Box<Range<usize>>),
    Range(Box<Range<Expr>>),
    Contains(String),
    Prefix(String),
    Suffix(String),
    Pattern(String),
    Custom(Box<Custom>),
    Inner(Box<Rule>),
}

pub enum Range<T> {
    GreaterThan(T),
    LowerThan(T),
    Between(T, T),
}

pub enum Custom {
    Fn(Path),
    Closure(ExprClosure),
}
