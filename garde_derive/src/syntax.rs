use std::collections::BTreeMap;

use proc_macro2::{Ident, Span};
use syn::ext::IdentExt;
use syn::parse::Parse;
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{DeriveInput, Token, Type};

use crate::model;
use crate::model::List;
use crate::util::MaybeFoldError;

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
                Ok(model::Attr::Context(Box::new(ty)))
            }
            "allow_unvalidated" => Ok(model::Attr::AllowUnvalidated),
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
            Ok(Some(v)) => variants.push((variant.ident.clone(), v)),
            Ok(None) => {}
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
                $($name:literal => $rule:ident $(($content:ident))?,)*
            }) => {
                match $ident.to_string().as_str() {
                    $(
                        $name => {
                            $(
                                let $content;
                                syn::parenthesized!($content in $input);
                            )?
                            Ok(model::RawRule {
                                span: $ident.span(),
                                kind: model::RawRuleKind::$rule $(($content.parse()?))?
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
                "rename" => Rename(content),
                "message" => Message(content),
                "code" => Code(content),
                "dive" => Dive,
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
                "byte_length" => ByteLength(content),
                "range" => Range(content),
                "contains" => Contains(content),
                "prefix" => Prefix(content),
                "suffix" => Suffix(content),
                "pattern" => Pattern(content),
                "custom" => Custom(content),
                "inner" => Inner(content),
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

impl Parse for model::Message {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(syn::LitStr) {
            Ok(Self::Fmt(model::Str::parse(input)?))
        } else {
            Ok(Self::Func(model::Func::parse(input)?))
        }
    }
}

impl Parse for model::Func {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let expr = syn::Expr::parse(input)?;
        match expr {
            syn::Expr::Closure(v) => Ok(model::Func::Closure(v)),
            syn::Expr::Path(v) => Ok(model::Func::Path(v)),
            _ => Err(syn::Error::new(expr.span(), "expected path or closure")),
        }
    }
}

impl<T> Parse for model::Range<T>
where
    T: FromExpr,
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let span = input.span();

        let pairs =
            syn::punctuated::Punctuated::<syn::MetaNameValue, Token![,]>::parse_terminated(input)?;

        let mut error = None;

        let mut min = None::<T>;
        let mut max = None::<T>;

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
            } else {
                error.maybe_fold(syn::Error::new(pair.path.span(), "unexpected argument"));
                continue;
            }
        }

        Ok(model::Range { span, min, max })
    }
}

impl<T: Parse> Parse for List<T> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let span = input.span();

        type CommaSeparated<T> = Punctuated<T, Token![,]>;
        let contents: Vec<_> = CommaSeparated::parse_terminated(input)?
            .into_iter()
            .collect();

        Ok(Self { span, contents })
    }
}

trait FromExpr: Sized {
    fn from_expr(v: syn::Expr) -> syn::Result<Self>;
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
