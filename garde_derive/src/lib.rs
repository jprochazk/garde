use std::collections::BTreeSet;

use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::{format_ident, quote, ToTokens};
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{
    Attribute, Data, DeriveInput, Error, Expr, ExprRange, Generics, MetaNameValue, Token, Type,
};

#[proc_macro_derive(Validate)]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    // TODO: remove known attributes from `input`
    let input = syn::parse_macro_input!(input as DeriveInput);

    let mut errors: Vec<Error> = vec![];

    let ident = input.ident.clone();
    let generics = input.generics.clone();
    let context = parse_context(&input.attrs, &mut errors);
    let inner = match parse_input_kind(&input.data, &mut errors) {
        Ok(inner) => inner,
        Err(e) => {
            errors.push(e);
            return emit_errors(input, errors);
        }
    };

    let validation = Validation {
        ident,
        generics,
        context,
        inner,
    };

    if !errors.is_empty() {
        return emit_errors(input, errors);
    }

    quote! {
        #input
    }
    .into()
}

fn emit_errors(input: DeriveInput, errors: Vec<Error>) -> TokenStream {
    let errors = errors.into_iter().map(|e| e.into_compile_error());

    quote! {
        #input

        #(
            #errors
        )*
    }
    .into()
}

struct Validation {
    ident: Ident,
    generics: Generics,
    context: Context,
    inner: InputKind,
}

impl ToTokens for Validation {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let ident = &self.ident;
        let context = &self.context;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        match &self.inner {
            InputKind::Struct(inner) => {
                let fields = inner.iter();

                quote! {
                    impl #impl_generics ::garde::Validate for #ident #ty_generics #where_clause {
                        type Context = #context;

                        fn validate(&self, ctx: Self::Context) -> Result<(), ::garde::Errors> {
                            let mut errors = ::garde::Errors::new();

                            #(#fields)*

                            if !errors.is_empty() {
                                return Err(errors);
                            }

                            Ok(())
                        }
                    }
                }
                .to_tokens(tokens)
            }
            InputKind::Enum(inner) => todo!(),
        }
    }
}

struct Context {
    inner: Option<Type>,
}

impl ToTokens for Context {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match &self.inner {
            Some(ty) => ty.to_tokens(tokens),
            None => quote!(()).to_tokens(tokens),
        }
    }
}

fn parse_context(attrs: &[Attribute], errors: &mut Vec<Error>) -> Context {
    let mut inner = None;
    for attr in attrs {
        if attr.path().is_ident("garde") {
            let meta = match attr.parse_args_with(MetaNameValue::parse) {
                Ok(meta) => meta,
                Err(e) => {
                    errors.push(e);
                    continue;
                }
            };
            if !meta.path.is_ident("context") {
                errors.push(Error::new(meta.span(), "unrecognized attribute"));
                continue;
            }
            if inner.is_some() {
                errors.push(Error::new(meta.span(), "duplicate context attribute"));
                continue;
            }
            let Expr::Path(path) = meta.value else {
                errors.push(Error::new(meta.span(), "invalid context type"));
                continue;
            };
            inner = Some(Type::Path(syn::TypePath {
                qself: None,
                path: path.path,
            }));
        }
    }
    Context { inner }
}

enum InputKind {
    Struct(Vec<Field>),
    Enum(Vec<Variant>),
}

fn parse_input_kind(data: &Data, errors: &mut Vec<Error>) -> syn::Result<InputKind> {
    match data {
        Data::Struct(v) => {
            let fields = match &v.fields {
                syn::Fields::Named(v) => parse_fields(false, v.named.iter(), errors),
                syn::Fields::Unnamed(v) => parse_fields(true, v.unnamed.iter(), errors),
                syn::Fields::Unit => vec![],
            };
            Ok(InputKind::Struct(fields))
        }
        Data::Enum(v) => {
            let variants = parse_variants(v.variants.iter(), errors);
            Ok(InputKind::Enum(variants))
        }
        Data::Union(v) => Err(Error::new(
            v.union_token.span().join(v.fields.span()).unwrap(),
            "unions are not supported",
        )),
    }
}

struct Variant {
    ident: Ident,
    fields: Vec<Field>,
}

fn parse_variants<'a>(
    variants: impl Iterator<Item = &'a syn::Variant>,
    errors: &mut Vec<Error>,
) -> Vec<Variant> {
    let mut out = vec![];

    for v in variants {
        let ident = v.ident.clone();
        let fields = match &v.fields {
            syn::Fields::Named(v) => parse_fields(false, v.named.iter(), errors),
            syn::Fields::Unnamed(v) => parse_fields(true, v.unnamed.iter(), errors),
            syn::Fields::Unit => vec![],
        };
        out.push(Variant { ident, fields })
    }

    out
}

struct Field {
    ident: Ident,
    // TODO: alias fields
    alias: Option<String>,
    ty: Type,
    rules: BTreeSet<Rule>,
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Field {
            ident,
            alias,
            ty,
            rules,
        } = self;

        quote! {
            if self.#ident
        }
        .to_tokens(tokens)
    }
}

fn parse_fields<'a>(
    unnamed: bool,
    fields: impl Iterator<Item = &'a syn::Field>,
    errors: &mut Vec<Error>,
) -> Vec<Field> {
    let mut out = vec![];

    for (i, f) in fields.enumerate() {
        let ident = if unnamed {
            format_ident!("{i}")
        } else {
            f.ident.clone().unwrap()
        };
        let ty = f.ty.clone();

        let mut rules = BTreeSet::new();
        for attr in f.attrs.iter() {
            if attr.path().is_ident("garde") {
                let mut rule = Rule::Ascii;
                if let Err(e) = attr.parse_nested_meta(|meta| {
                    rule = Rule::parse(meta.input)?;
                    Ok(())
                }) {
                    errors.push(e);
                    continue;
                }

                if rules.contains(&rule) {
                    errors.push(Error::new(
                        attr.meta.span(),
                        format!("duplicate rule {}", rule.name()),
                    ));
                    continue;
                }

                rules.insert(rule);
            }
        }

        // TODO: validate rules

        out.push(Field {
            ident,
            alias: None,
            ty,
            rules,
        })
    }

    out
}

// TODO: some of these rules should only exist with their respective features
#[repr(u8)]
enum Rule {
    Ascii,
    Alphanumeric,
    Email,
    Url,
    Ip,
    IpV4,
    IpV6,
    CreditCard,
    PhoneNumber,
    Length {
        min: Option<usize>,
        max: Option<usize>,
    },
    Contains(String),
    StartsWith(String),
    EndsWith(String),
    Regex(String),
    Custom(Expr),
}

impl Rule {
    fn emit(&self, field_name: &Ident, tokens: &mut proc_macro2::TokenStream) {
        let path = match self {
            Rule::Ascii => quote!(::garde::rules::ascii),
            Rule::Alphanumeric => quote!(::garde::rules::alphanumeric),
            Rule::Email => quote!(::garde::rules::email),
            Rule::Url => quote!(::garde::rules::url),
            Rule::Ip => quote!(::garde::rules::ip),
            Rule::IpV4 => quote!(::garde::rules::ipv4),
            Rule::IpV6 => quote!(::garde::rules::ipv6),
            Rule::CreditCard => quote!(::garde::rules::credit_card),
            Rule::PhoneNumber => quote!(::garde::rules::phone_number),
            Rule::Length { min, max } => quote!(::garde::rules::length),
            Rule::Contains(_) => quote!(::garde::rules::contains),
            Rule::StartsWith(_) => quote!(::garde::rules::starts_with),
            Rule::EndsWith(_) => quote!(::garde::rules::ends_with),
            Rule::Regex(_) => quote!(::garde::rules::regex),
            Rule::Custom(_) => quote!(::garde::rules::custom),
        };
        quote!(#path::validate(&self.#field_name)).to_tokens(tokens)
    }
}

impl Rule {
    pub fn name(&self) -> &'static str {
        match self {
            Rule::Ascii => "ascii",
            Rule::Alphanumeric => "alphanumeric",
            Rule::Email => "email",
            Rule::Url => "url",
            Rule::Ip => "ip",
            Rule::IpV4 => "ipv4",
            Rule::IpV6 => "ipv6",
            Rule::CreditCard => "credit_card",
            Rule::PhoneNumber => "phone_number",
            Rule::Length { .. } => "length",
            Rule::Contains(_) => "contains",
            Rule::StartsWith(_) => "starts_with",
            Rule::EndsWith(_) => "ends_with",
            Rule::Regex(_) => "regex",
            Rule::Custom(_) => "custom",
        }
    }
}

impl Parse for Rule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = Ident::parse_any(input)?;

        macro_rules! parse_rule {
            ($ident:ident, $input:ident, $name:literal $body:block) => {
                if $ident == $name {
                    if !$input.is_empty() {
                        return Err(Error::new(
                            $ident.span(),
                            format!("{} does not accept any args", $name),
                        ));
                    }
                    return Ok($body);
                }
            };
            ($ident:ident, $input:ident, $name:literal ($content:ident) $body:block) => {
                if $ident == $name {
                    if $input.is_empty() {
                        return Err(Error::new(
                            $ident.span(),
                            format!("{} expects arguments", $name),
                        ));
                    }
                    let content;
                    syn::parenthesized!(content in input);
                    let $content = content;
                    return Ok($body);
                }
            };
        }

        parse_rule!(ident, input, "ascii" { Rule::Ascii });
        parse_rule!(ident, input, "alphanumeric" { Rule::Alphanumeric });
        parse_rule!(ident, input, "email" { Rule::Email });
        parse_rule!(ident, input, "url" { Rule::Url });
        parse_rule!(ident, input, "ip" { Rule::Ip });
        parse_rule!(ident, input, "ipv4" { Rule::IpV4 });
        parse_rule!(ident, input, "ipv6" { Rule::IpV6 });
        parse_rule!(ident, input, "credit_card" { Rule::CreditCard });
        parse_rule!(ident, input, "phone_number" { Rule::PhoneNumber });
        parse_rule!(ident, input, "length" (content) {
            parse_rule_length(&content)?
        });
        parse_rule!(ident, input, "contains" (content) {
            parse_rule_contains(&content)?
        });
        parse_rule!(ident, input, "starts_with" (content) {
            parse_rule_starts_with(&content)?
        });
        parse_rule!(ident, input, "ends_with" (content) {
            parse_rule_ends_with(&content)?
        });
        parse_rule!(ident, input, "regex" (content) {
            parse_rule_regex(&content)?
        });
        parse_rule!(ident, input, "custom" (content) {
            parse_rule_custom(&content)?
        });

        Err(Error::new(ident.span(), "unrecognized validation rule"))
    }
}

fn parse_rule_length(content: ParseStream) -> syn::Result<Rule> {
    let parts = content.parse_terminated(MetaNameValue::parse, Token![,])?;
    let mut min = None::<usize>;
    let mut max = None::<usize>;
    for part in parts.iter() {
        if part.path.is_ident("min") {
            if min.is_some() {
                return Err(Error::new(part.span(), "duplicate attribute"));
            }
            let value = match &part.value {
                Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Int(int),
                    ..
                }) => int.base10_parse::<usize>()?,
                _ => {
                    return Err(Error::new(
                        part.value.span(),
                        "value must be a valid `usize`",
                    ))
                }
            };
            min = Some(value)
        } else if part.path.is_ident("max") {
            if max.is_some() {
                return Err(Error::new(part.span(), "duplicate attribute"));
            }
            let value = match &part.value {
                Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Int(int),
                    ..
                }) => int.base10_parse::<usize>()?,
                _ => {
                    return Err(Error::new(
                        part.value.span(),
                        "value must be a valid `usize`",
                    ))
                }
            };
            max = Some(value)
        } else {
            return Err(Error::new(
                part.value.span(),
                "value must be a valid `usize`",
            ));
        }
    }
    match (min, max) {
        (Some(min), Some(max)) if min >= max => {
            return Err(Error::new(parts.span(), "min must be smaller than max"));
        }
        _ => {}
    }
    Ok(Rule::Length { min, max })
}
fn parse_rule_contains(content: ParseStream) -> syn::Result<Rule> {
    let content_span = content.span();
    let value = <syn::LitStr as Parse>::parse(content)?.value();
    if value.is_empty() {
        return Err(Error::new(content_span, "string must not be empty"));
    }
    Ok(Rule::Contains(value))
}
fn parse_rule_starts_with(content: ParseStream) -> syn::Result<Rule> {
    let content_span = content.span();
    let value = <syn::LitStr as Parse>::parse(content)?.value();
    if value.is_empty() {
        return Err(Error::new(content_span, "string must not be empty"));
    }
    Ok(Rule::StartsWith(value))
}
fn parse_rule_ends_with(content: ParseStream) -> syn::Result<Rule> {
    let content_span = content.span();
    let value = <syn::LitStr as Parse>::parse(content)?.value();
    if value.is_empty() {
        return Err(Error::new(content_span, "string must not be empty"));
    }
    Ok(Rule::EndsWith(value))
}
fn parse_rule_regex(content: ParseStream) -> syn::Result<Rule> {
    let content_span = content.span();
    let value = <syn::LitStr as Parse>::parse(content)?.value();
    if value.is_empty() {
        return Err(Error::new(content_span, "string must not be empty"));
    }
    Ok(Rule::Regex(value))
}
fn parse_rule_custom(content: ParseStream) -> syn::Result<Rule> {
    let expr = <syn::LitStr as Parse>::parse(content)?.parse::<Expr>()?;
    Ok(Rule::Custom(expr))
}

impl PartialEq for Rule {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Eq for Rule {}

impl Rule {
    fn discriminant(&self) -> u8 {
        // SAFETY: Because `Self` is marked `repr(u8)`, its layout is a `repr(C)`
        // `union` between `repr(C)` structs, each of which has the `u8`
        // discriminant as its first field, so we can read the discriminant
        // without offsetting the pointer.
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

impl PartialOrd for Rule {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.discriminant().partial_cmp(&other.discriminant())
    }
}

impl Ord for Rule {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.discriminant().cmp(&other.discriminant())
    }
}
