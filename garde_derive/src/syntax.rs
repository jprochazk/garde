use std::collections::BTreeMap;

use proc_macro2::{Ident, Span};
use syn::ext::IdentExt;
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::token::As;
use syn::{DeriveInput, Expr, Token, Type};

use crate::model;
use crate::model::List;
use crate::util::{default_ctx_name, MaybeFoldError};

pub fn parse(input: DeriveInput) -> syn::Result<model::Input> {
    let mut error = None;

    let ident = input.ident.clone();
    let generics = input.generics.clone();
    let attrs = match parse_input_attr_list(&input.attrs) {
        Ok(v) => v,
        Err(e) => {
            error.maybe_fold(e);
            Vec::new()
        }
    };
    let kind = match &input.data {
        syn::Data::Struct(v) => parse_struct(v),
        syn::Data::Enum(v) => parse_enum(v),
        syn::Data::Union(v) => parse_union(v),
    };
    let kind = match kind {
        Ok(kind) => kind,
        Err(e) => {
            error.maybe_fold(e);
            model::InputKind::empty()
        }
    };

    if let Some(error) = error {
        return Err(error);
    }

    Ok(model::Input {
        ident,
        generics,
        attrs,
        kind,
    })
}

fn parse_input_attr_list(attrs: &[syn::Attribute]) -> syn::Result<Vec<(Span, model::Attr)>> {
    let mut error = None;
    let mut out = Vec::new();

    for attr in attrs.iter() {
        if attr.path().is_ident("garde") {
            match parse_input_attr(attr) {
                Ok(v) => out.push((attr.span(), v)),
                Err(e) => error.maybe_fold(e),
            }
        }
    }

    if let Some(error) = error {
        return Err(error);
    }

    Ok(out)
}

fn parse_input_attr(attr: &syn::Attribute) -> syn::Result<model::Attr> {
    let meta_list = match attr.meta.require_list() {
        Ok(v) => v,
        Err(_) => {
            return Err(syn::Error::new(
                attr.meta.span(),
                "invalid attr style, expected parenthesized arguments",
            ))
        }
    };

    syn::parse2::<model::Attr>(meta_list.tokens.clone())
}

impl Parse for model::Attr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = Ident::parse_any(input)?;
        match ident.to_string().as_str() {
            "context" => {
                let content;
                syn::parenthesized!(content in input);
                let ty = content.parse::<Type>()?;
                let ident = if content.parse::<As>().is_ok() {
                    content.parse()?
                } else {
                    default_ctx_name()
                };

                Ok(model::Attr::Context(Box::new(ty), ident))
            }
            "allow_unvalidated" => Ok(model::Attr::AllowUnvalidated),
            "transparent" => Ok(model::Attr::Transparent),
            _ => Err(syn::Error::new(ident.span(), "unrecognized attribute")),
        }
    }
}

fn parse_struct(node: &syn::DataStruct) -> syn::Result<model::InputKind> {
    let mut error = None;

    let fields = match parse_variant(&node.fields) {
        Ok(Some(v)) => v,
        Ok(None) => {
            error.maybe_fold(syn::Error::new(
                node.fields.span(),
                "unit structs are unsupported",
            ));
            model::Variant::empty()
        }
        Err(e) => {
            error.maybe_fold(e);
            model::Variant::empty()
        }
    };

    if let Some(error) = error {
        return Err(error);
    }

    Ok(model::InputKind::Struct(fields))
}

fn parse_enum(node: &syn::DataEnum) -> syn::Result<model::InputKind> {
    let mut error = None;
    let mut variants = Vec::new();

    for variant in node.variants.iter() {
        match parse_variant(&variant.fields) {
            Ok(v) => variants.push((variant.ident.clone(), v)),
            Err(e) => error.maybe_fold(e),
        }
    }

    if let Some(error) = error {
        return Err(error);
    }

    Ok(model::InputKind::Enum(variants))
}

fn parse_union(node: &syn::DataUnion) -> syn::Result<model::InputKind> {
    Err(syn::Error::new(
        node.union_token.span(),
        "unions are unsupported",
    ))
}

fn parse_variant(fields: &syn::Fields) -> syn::Result<Option<model::Variant>> {
    let mut error = None;

    let variant = match fields {
        syn::Fields::Named(v) => {
            let mut fields = BTreeMap::new();
            for field in v.named.iter() {
                let ident = field.ident.clone().unwrap();
                let ty = field.ty.clone();
                let rules = match parse_field_attr_list(&field.attrs) {
                    Ok(v) => v,
                    Err(e) => {
                        error.maybe_fold(e);
                        Vec::new()
                    }
                };
                fields.insert(ident, model::Field { ty, rules });
            }
            Some(model::Variant::Struct(fields))
        }
        syn::Fields::Unnamed(v) => {
            let mut fields = Vec::new();
            for field in v.unnamed.iter() {
                let ty = field.ty.clone();
                let rules = match parse_field_attr_list(&field.attrs) {
                    Ok(v) => v,
                    Err(e) => {
                        error.maybe_fold(e);
                        Vec::new()
                    }
                };
                fields.push(model::Field { ty, rules });
            }
            Some(model::Variant::Tuple(fields))
        }
        syn::Fields::Unit => None,
    };

    if let Some(error) = error {
        return Err(error);
    }

    Ok(variant)
}

fn parse_field_attr_list(attrs: &[syn::Attribute]) -> syn::Result<Vec<model::RawRule>> {
    let mut error = None;
    let mut rules = Vec::new();

    for attr in attrs.iter() {
        if attr.path().is_ident("garde") {
            match attr.parse_args_with(Punctuated::<_, syn::token::Comma>::parse_terminated) {
                Ok(list) => {
                    for rule in list {
                        match rule {
                            ContinueOnFail::Ok(v) => rules.push(v),
                            ContinueOnFail::Err(e) => error.maybe_fold(e),
                        }
                    }
                }
                Err(e) => error.maybe_fold(e),
            }
        }
    }

    if let Some(error) = error {
        return Err(error);
    }

    Ok(rules)
}

enum ContinueOnFail<T> {
    Ok(T),
    Err(syn::Error),
}

impl<T> Parse for ContinueOnFail<T>
where
    T: Parse,
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        match <T as Parse>::parse(input) {
            Ok(v) => Ok(ContinueOnFail::Ok(v)),
            Err(e) => Ok(ContinueOnFail::Err(e)),
        }
    }
}

impl Parse for model::RawRule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = Ident::parse_any(input)?;

        macro_rules! rules {
            (($input:ident, $ident:ident) {
                $($name:literal => $rule:ident $(($content:ident))? $(( ? $content_opt:ident))?,)*
            }) => {
                match $ident.to_string().as_str() {
                    $(
                        $name => {
                            $(
                                let $content;
                                syn::parenthesized!($content in $input);
                                let $content = $content.parse()?;
                            )?
                            $(
                                let $content_opt = if $input.peek(syn::token::Paren) {
                                    let $content_opt;
                                    syn::parenthesized!($content_opt in $input);
                                    if $content_opt.is_empty() {
                                        None
                                    } else {
                                        Some($content_opt.parse()?)
                                    }
                                } else {
                                    None
                                };
                            )?
                            Ok(model::RawRule {
                                span: $ident.span(),
                                kind: model::RawRuleKind::$rule $(($content))? $(($content_opt))?
                            })
                        }
                    )*
                    _ => Err(syn::Error::new($ident.span(), "unrecognized validation rule")),
                }
            };
        }

        rules! {
            (input, ident) {
                "skip" => Skip,
                "adapt" => Adapt(content),
                "rename" => Rename(content),
                // "message" => Message(content),
                "code" => Code(content),
                "dive" => Dive(? content),
                "required" => Required,
                "ascii" => Ascii,
                "alphanumeric" => Alphanumeric,
                "email" => Email,
                "url" => Url,
                "ip" => Ip,
                "ipv4" => IpV4,
                "ipv6" => IpV6,
                "credit_card" => CreditCard,
                "phone_number" => PhoneNumber,
                "length" => Length(content),
                "matches" => Matches(content),
                "range" => Range(content),
                "contains" => Contains(content),
                "prefix" => Prefix(content),
                "suffix" => Suffix(content),
                "pattern" => Pattern(content),
                "custom" => Custom(content),
                "inner" => Inner(content),
                "if" => If(content),
            }
        }
    }
}

impl Parse for model::Pattern {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::Lit) {
            Ok(Self::Lit(model::Str::parse(input)?))
        } else {
            Ok(Self::Expr(syn::Expr::parse(input)?))
        }
    }
}

impl Parse for model::Str {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(model::Str {
            span: input.span(),
            value: <syn::LitStr as Parse>::parse(input)?.value(),
        })
    }
}

// impl Parse for model::Message {
//     fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//         if input.peek(syn::LitStr) {
//             Ok(Self::Fmt(model::Str::parse(input)?))
//         } else {
//             Ok(Self::Func(syn::Expr::parse(input)?))
//         }
//     }
// }

impl Parse for model::RawLength {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let span = input.span();

        let args =
            Punctuated::<ContinueOnFail<RawLengthArgument>, Token![,]>::parse_terminated(input)?;

        let mut error = None;

        let mut mode = None;
        let mut min = None;
        let mut max = None;
        let mut equal = None;

        for arg in args {
            let arg = match arg {
                ContinueOnFail::Ok(arg) => arg,
                ContinueOnFail::Err(e) => {
                    error.maybe_fold(e);
                    continue;
                }
            };
            match arg {
                RawLengthArgument::Min(span, v) => {
                    if min.is_some() {
                        error.maybe_fold(syn::Error::new(span, "duplicate argument"))
                    } else {
                        min = Some(v)
                    }
                }
                RawLengthArgument::Max(span, v) => {
                    if max.is_some() {
                        error.maybe_fold(syn::Error::new(span, "duplicate argument"))
                    } else {
                        max = Some(v)
                    }
                }
                RawLengthArgument::Equal(span, v) => {
                    if equal.is_some() {
                        error.maybe_fold(syn::Error::new(span, "duplicate argument"))
                    } else {
                        equal = Some(v)
                    }
                }
                RawLengthArgument::Mode(span, v) => {
                    if mode.is_some() {
                        error.maybe_fold(syn::Error::new(span, "duplicate argument"))
                    } else {
                        mode = Some(v)
                    }
                }
            }
        }

        if let Some(error) = error {
            return Err(error);
        }

        Ok(model::RawLength {
            mode: mode.unwrap_or_default(),
            range: model::Range {
                span,
                min,
                max,
                equal,
            },
        })
    }
}

enum RawLengthArgument {
    Min(Span, model::Either<usize, syn::Expr>),
    Max(Span, model::Either<usize, syn::Expr>),
    Equal(Span, model::Either<usize, syn::Expr>),
    Mode(Span, model::LengthMode),
}

impl Parse for RawLengthArgument {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = Ident::parse_any(input)?;
        let span = ident.span();
        let v = match ident.to_string().as_str() {
            "simple" => RawLengthArgument::Mode(span, model::LengthMode::Simple),
            "bytes" => RawLengthArgument::Mode(span, model::LengthMode::Bytes),
            "chars" => RawLengthArgument::Mode(span, model::LengthMode::Chars),
            "graphemes" => RawLengthArgument::Mode(span, model::LengthMode::Graphemes),
            "utf16" => RawLengthArgument::Mode(span, model::LengthMode::Utf16),
            "min" => {
                let _ = input.parse::<Token![=]>()?;
                let v = input.parse::<syn::Expr>()?;
                RawLengthArgument::Min(span, FromExpr::from_expr(v)?)
            }
            "max" => {
                let _ = input.parse::<Token![=]>()?;
                let v = input.parse::<syn::Expr>()?;
                RawLengthArgument::Max(span, FromExpr::from_expr(v)?)
            }
            "equal" => {
                let _ = input.parse::<Token![=]>()?;
                let v = input.parse::<syn::Expr>()?;
                RawLengthArgument::Equal(span, FromExpr::from_expr(v)?)
            }
            _ => {
                if input.peek(Token![=]) {
                    let _ = input.parse::<Token![=]>()?;
                }
                if !input.peek(Token![,]) {
                    let _ = input.parse::<syn::Expr>()?;
                }
                return Err(syn::Error::new(span, "invalid argument"));
            }
        };
        Ok(v)
    }
}

impl<T> Parse for model::Range<T>
where
    T: FromExpr,
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let span = input.span();

        let pairs = Punctuated::<syn::MetaNameValue, Token![,]>::parse_terminated(input)?;

        let mut error = None;

        let mut min = None::<T>;
        let mut max = None::<T>;
        let mut equal = None::<T>;

        for pair in pairs {
            if pair.path.is_ident("min") {
                if min.is_some() {
                    error.maybe_fold(syn::Error::new(pair.path.span(), "duplicate argument"));
                    continue;
                }
                let value = match <T as FromExpr>::from_expr(pair.value) {
                    Ok(v) => v,
                    Err(e) => {
                        error.maybe_fold(e);
                        continue;
                    }
                };
                min = Some(value);
            } else if pair.path.is_ident("max") {
                if max.is_some() {
                    error.maybe_fold(syn::Error::new(pair.path.span(), "duplicate argument"));
                    continue;
                }
                let value = match <T as FromExpr>::from_expr(pair.value) {
                    Ok(v) => v,
                    Err(e) => {
                        error.maybe_fold(e);
                        continue;
                    }
                };
                max = Some(value);
            } else if pair.path.is_ident("equal") {
                if equal.is_some() {
                    error.maybe_fold(syn::Error::new(pair.path.span(), "duplicate argument"));
                    continue;
                }
                let value = match <T as FromExpr>::from_expr(pair.value) {
                    Ok(v) => v,
                    Err(e) => {
                        error.maybe_fold(e);
                        continue;
                    }
                };

                if min.is_some() || max.is_some() {
                    error.maybe_fold(syn::Error::new(
                        pair.path.span(),
                        "min or max conflict with equal",
                    ));
                }
                equal = Some(value);
            } else {
                error.maybe_fold(syn::Error::new(pair.path.span(), "unexpected argument"));
                continue;
            }
        }

        if let Some(error) = error {
            Err(error)
        } else {
            Ok(model::Range {
                span,
                min,
                max,
                equal,
            })
        }
    }
}

impl<T: Parse> Parse for List<T> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        type CommaSeparated<T> = Punctuated<T, Token![,]>;
        let contents: Vec<_> = CommaSeparated::parse_terminated(input)?
            .into_iter()
            .collect();

        Ok(Self { contents })
    }
}

impl Parse for model::IfRule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        // Parse "cond = <expr>,"
        let cond_ident = input.parse::<Ident>()?;
        if cond_ident != "cond" {
            return Err(syn::Error::new(
                cond_ident.span(),
                "expected 'cond' in if rule",
            ));
        }
        input.parse::<Token![=]>()?;
        let condition = input.parse::<Expr>()?;

        // Expect comma after condition
        input.parse::<Token![,]>()?;

        // Parse the remaining rules
        let rules = input.parse::<List<model::RawRule>>()?;

        if rules.contents.is_empty() {
            return Err(syn::Error::new(
                input.span(),
                "if rule must contain at least one validation rule",
            ));
        }

        Ok(model::IfRule { condition, rules })
    }
}

trait FromExpr: Sized {
    fn from_expr(v: syn::Expr) -> syn::Result<Self>;
}

impl<L, R> FromExpr for model::Either<L, R>
where
    L: FromExpr,
    R: FromExpr,
{
    fn from_expr(v: syn::Expr) -> syn::Result<Self> {
        L::from_expr(v.clone())
            .map(model::Either::Left)
            .or_else(|_| R::from_expr(v).map(model::Either::Right))
    }
}

impl FromExpr for syn::Expr {
    fn from_expr(v: syn::Expr) -> syn::Result<Self> {
        Ok(v)
    }
}

impl FromExpr for usize {
    fn from_expr(v: syn::Expr) -> syn::Result<Self> {
        match v {
            syn::Expr::Lit(syn::ExprLit {
                lit: syn::Lit::Int(v),
                ..
            }) => v.base10_parse(),
            _ => Err(syn::Error::new(v.span(), "expected usize")),
        }
    }
}
