mod check;
mod emit;
mod model;
mod syntax;

use std::collections::BTreeSet;
use std::fmt::Display;
use std::str::FromStr;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Literal, Span, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{
    Attribute, Data, DeriveInput, Error, Expr, GenericParam, Generics, MetaNameValue, Token, Type,
};

/* #[doc(hidden)]
fn derive_validate2(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    macro_rules! c {
        ($expr) => {
            match $expr {
                Ok(v) => v,
                Err(e) => return e.into_compile_error().into(),
            }
        }
    }

    let input = c!(syntax::parse(input));

    let impl_ = match parse(&input) {
        Ok(v) => v,
        Err(e) => return e.into_compile_error().into(),
    };

    quote! {
        impl_
    }
    .into()
} */

#[proc_macro_derive(Validate, attributes(garde))]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);

    let mut errors: Vec<Error> = vec![];

    let ident = input.ident.clone();
    let generics = input.generics.clone();
    let context = parse_context(&input.attrs, &mut errors);
    let inner = match parse_input_kind(&input.data, &mut errors) {
        Ok(inner) => inner,
        Err(e) => {
            errors.push(e);
            return emit_errors(errors);
        }
    };

    let validate_impl = Validation {
        ident,
        generics,
        context,
        inner,
    };

    if !errors.is_empty() {
        return emit_errors(errors);
    }

    quote! {
        #validate_impl
    }
    .into()
}

fn emit_errors(errors: Vec<Error>) -> TokenStream {
    let errors = errors.into_iter().map(|e| e.into_compile_error());

    quote! {
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
        let (context_ty, _ /* context_generics */) = self.context.split_for_impl();
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        /* let mut impl_generics = self.generics.clone();
        if let Some(context_generics) = context_generics {
            for param in context_generics.iter().cloned() {
                impl_generics.params.push(param);
            }
        }
        let (impl_generics, _, _) = impl_generics.split_for_impl(); */

        let inner = match &self.inner {
            InputKind::FieldStruct(inner) => {
                let fields = inner
                    .iter()
                    .map(|(key, field)| EmitField::FieldStruct(key, field));

                quote! {
                    ::garde::error::Errors::fields(|__garde_errors| {
                        #(#fields)*
                    })
                }
            }
            InputKind::TupleStruct(inner) => {
                let fields = inner
                    .iter()
                    .enumerate()
                    .map(|(i, field)| EmitField::TupleStruct(i, field));

                quote! {
                    ::garde::error::Errors::list(|__garde_errors| {
                        #(#fields)*
                    })
                }
            }
            InputKind::Enum(inner) => {
                let variants = inner.iter();

                quote! {
                    match self {
                        #(#variants)*
                    }
                }
            }
        };

        quote! {
            impl #impl_generics ::garde::Validate for #ident #ty_generics #where_clause {
                type Context = #context_ty;

                fn validate(&self, __garde_user_ctx: &Self::Context) -> ::core::result::Result<(), ::garde::error::Errors> {
                    let __garde_errors = #inner ;

                    if !__garde_errors.is_empty() {
                        return Err(__garde_errors);
                    }

                    Ok(())
                }
            }
        }
        .to_tokens(tokens)
    }
}

struct Context {
    inner: ContextMeta,
}

impl Context {
    fn split_for_impl(&self) -> (&Type, Option<&Punctuated<GenericParam, Token![,]>>) {
        (&self.inner.ty, self.inner.params.as_ref())
    }
}

fn parse_context(attrs: &[Attribute], errors: &mut Vec<Error>) -> Context {
    let mut inner = None;
    for attr in attrs {
        if attr.path().is_ident("garde") {
            let ty = match attr.parse_args_with(parse_context_meta) {
                Ok(ty) => ty,
                Err(e) => {
                    errors.push(e);
                    continue;
                }
            };
            if inner.is_some() {
                errors.push(Error::new(
                    attr.path().span(),
                    "duplicate attribute `context`",
                ));
                continue;
            }
            inner = Some(ty);
        }
    }
    let inner = match inner {
        Some(inner) => inner,
        None => ContextMeta {
            ty: syn::parse_quote!(()),
            params: None,
        },
    };
    Context { inner }
}

struct ContextMeta {
    params: Option<Punctuated<GenericParam, Token![,]>>,
    ty: Type,
}

fn parse_context_meta(input: ParseStream) -> syn::Result<ContextMeta> {
    let name = Ident::parse_any(input)?;
    if name != "context" {
        return Err(Error::new(name.span(), "unrecognized attribute"));
    }
    let content;
    syn::parenthesized!(content in input);

    let params = None;
    /* let params = if content.peek(Token![for]) {
        <Token![for]>::parse(&content)?;
        <Token![<]>::parse(&content)?;
        let params = Punctuated::parse_separated_nonempty(&content)?;
        <Token![>]>::parse(&content)?;
        Some(params)
    } else {
        None
    }; */

    let ty = content.parse::<Type>()?;

    Ok(ContextMeta { params, ty })
}

enum InputKind {
    FieldStruct(Vec<(Ident, Field)>),
    TupleStruct(Vec<Field>),
    Enum(Vec<Variant>),
}

fn parse_input_kind(data: &Data, errors: &mut Vec<Error>) -> syn::Result<InputKind> {
    match data {
        Data::Struct(v) => match &v.fields {
            syn::Fields::Named(v) => Ok(InputKind::FieldStruct(
                parse_fields(v.named.iter(), errors)
                    .into_iter()
                    .map(|(ident, field)| (ident.unwrap(), field))
                    .collect(),
            )),
            syn::Fields::Unnamed(v) => Ok(InputKind::TupleStruct(
                parse_fields(v.unnamed.iter(), errors)
                    .into_iter()
                    .map(|(_, field)| (field))
                    .collect(),
            )),
            syn::Fields::Unit => Ok(InputKind::TupleStruct(vec![])),
        },
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
    kind: VariantKind,
}

enum VariantKind {
    Unit,
    Struct(Vec<(Ident, Field)>),
    Tuple(Vec<Field>),
}

impl ToTokens for VariantKind {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            VariantKind::Unit => quote! {=> {}}.to_tokens(tokens),
            VariantKind::Struct(fields) => {
                let mut bindings = Vec::with_capacity(fields.len());
                let mut checks = Vec::with_capacity(fields.len());
                for (key, field) in fields.iter() {
                    if field.skip {
                        continue;
                    }
                    bindings.push(key);
                    checks.push(EmitField::FieldEnum(key, field));
                }
                let rest = if bindings.len() != fields.len() {
                    Some(quote!(..))
                } else {
                    None
                };
                quote! {
                    {#(#bindings,)* #rest} => ::garde::error::Errors::fields(|__garde_errors| {
                        #(#checks)*
                    }),
                }
                .to_tokens(tokens)
            }
            VariantKind::Tuple(fields) => {
                let mut bindings = Vec::with_capacity(fields.len());
                let mut checks = Vec::with_capacity(fields.len());
                for (i, field) in fields.iter().enumerate() {
                    if field.skip {
                        continue;
                    }
                    let field_name = format_ident!("_{i}");
                    bindings.push(field_name.clone());
                    checks.push(EmitField::TupleEnum(field_name, field))
                }
                let rest = if bindings.len() != fields.len() {
                    Some(quote!(..))
                } else {
                    None
                };
                quote! {
                    (#(#bindings,)* #rest) => ::garde::error::Errors::list(|__garde_errors| {
                        #(#checks)*
                    }),
                }
                .to_tokens(tokens)
            }
        }
    }
}

enum EmitField<'a> {
    FieldStruct(&'a Ident, &'a Field),
    TupleStruct(usize, &'a Field),
    FieldEnum(&'a Ident, &'a Field),
    TupleEnum(Ident, &'a Field),
}

impl<'a> ToTokens for EmitField<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            EmitField::FieldStruct(key, v) => v.emit(FieldEmitKind::FieldStruct(key), tokens),
            EmitField::TupleStruct(index, v) => v.emit(
                FieldEmitKind::TupleStruct(&Literal::usize_unsuffixed(*index)),
                tokens,
            ),
            EmitField::FieldEnum(key, v) => v.emit(FieldEmitKind::FieldEnum(key), tokens),
            EmitField::TupleEnum(index, v) => v.emit(FieldEmitKind::TupleEnum(index), tokens),
        }
    }
}

impl ToTokens for Variant {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Variant { ident, kind } = self;

        quote! {Self::#ident #kind}.to_tokens(tokens)
    }
}

fn parse_variants<'a>(
    variants: impl Iterator<Item = &'a syn::Variant>,
    errors: &mut Vec<Error>,
) -> Vec<Variant> {
    let mut out = vec![];

    for v in variants {
        let ident = v.ident.clone();
        let kind = match &v.fields {
            syn::Fields::Named(v) => VariantKind::Struct(
                parse_fields(v.named.iter(), errors)
                    .into_iter()
                    .map(|(ident, field)| (ident.unwrap(), field))
                    .collect(),
            ),
            syn::Fields::Unnamed(v) => VariantKind::Tuple(
                parse_fields(v.unnamed.iter(), errors)
                    .into_iter()
                    .map(|(_, field)| (field))
                    .collect(),
            ),
            syn::Fields::Unit => VariantKind::Unit,
        };
        out.push(Variant { ident, kind })
    }

    out
}

struct Field {
    ty: Type,
    dive: bool,
    skip: bool,
    rules: BTreeSet<Rule>,
}

enum FieldEmitKind<'a> {
    FieldStruct(&'a Ident),
    TupleStruct(&'a Literal),
    FieldEnum(&'a Ident),
    TupleEnum(&'a Ident),
}

impl Field {
    fn emit(&self, kind: FieldEmitKind, tokens: &mut TokenStream2) {
        if self.skip {
            return;
        }

        let (access, key, add_fn) = match &kind {
            FieldEmitKind::FieldStruct(key) => {
                (quote!(&self.#key), Some(key.to_string()), quote!(insert))
            }
            FieldEmitKind::TupleStruct(index) => (quote!(&self.#index), None, quote!(push)),
            FieldEmitKind::FieldEnum(key) => (quote!(#key), Some(key.to_string()), quote!(insert)),
            FieldEmitKind::TupleEnum(index) => (quote!(#index), None, quote!(push)),
        };

        let error =
            match self.dive {
                true => quote!(
                    ::garde::validate::Validate::validate(#access, __garde_user_ctx)
                        .err()
                        .unwrap_or_else(|| ::garde::error::Errors::empty())
                ),
                false => {
                    let rules = self.rules.iter().map(|rule| rule.emit(self)).map(
                        |RuleEmit { rule, args }| {
                            quote! {
                                if let Err(__garde_error) = (#rule)(#access, #args) {
                                    __garde_errors.push(__garde_error)
                                }
                            }
                        },
                    );
                    quote!(::garde::error::Errors::simple(|__garde_errors| {#(#rules)*}))
                }
            };

        let key = key.map(|key| quote!(#key,));
        quote! {
            __garde_errors.#add_fn(#key #error);
        }
        .to_tokens(tokens)
    }
}

fn parse_fields<'a>(
    fields: impl Iterator<Item = &'a syn::Field>,
    errors: &mut Vec<Error>,
) -> Vec<(Option<Ident>, Field)> {
    let mut out = vec![];

    for field in fields {
        let ident = field.ident.clone();
        let ty = field.ty.clone();

        // TODO: refactor this to not be so deeply nested

        let mut skip = false;
        let mut dive = false;
        let mut alias = None;
        let mut rules = BTreeSet::new();

        for attr in field.attrs.iter() {
            if attr.path().is_ident("garde") {
                let meta_list = match attr
                    .parse_args_with(Punctuated::<RuleOrAttr, Token![,]>::parse_terminated)
                {
                    Ok(rule) => rule,
                    Err(e) => {
                        errors.push(e);
                        continue;
                    }
                };

                for meta in meta_list {
                    match meta {
                        RuleOrAttr::Rule(span, rule) => {
                            if dive {
                                errors.push(Error::new(
                                    span,
                                    format!(
                                        "`{}` may not be used together with `dive`",
                                        rule.name()
                                    ),
                                ));
                                continue;
                            }

                            if rules.contains(&rule) {
                                errors.push(Error::new(
                                    span,
                                    format!("duplicate rule `{}`", rule.name()),
                                ));
                                continue;
                            }

                            rules.insert(rule);
                        }
                        RuleOrAttr::Attr(span, v) => match v {
                            // TODO: not allowed on tuple structs
                            Attr::Alias(v) => {
                                if alias.is_some() {
                                    errors.push(Error::new(span, "duplicate attribute `rename`"));
                                    continue;
                                }
                                alias = Some(v);
                            }
                            Attr::Dive => {
                                if dive {
                                    errors.push(Error::new(span, "duplicate attribute `dive`"));
                                    continue;
                                }
                                dive = true;
                            }
                            Attr::Skip => {
                                if skip {
                                    errors.push(Error::new(span, "duplicate attribute `skip`"));
                                    continue;
                                }
                                skip = true;
                            }
                        },
                        RuleOrAttr::Error(error) => {
                            errors.push(error);
                            continue;
                        }
                    }
                }
            }
        }

        if !dive && rules.is_empty() && !skip {
            errors.push(Error::new(
                field.ty.span(),
                "field has no validation, use `#[garde(skip)]` if this is intentional",
            ));
            continue;
        }

        out.push((
            ident,
            Field {
                ty,
                dive,
                skip,
                rules,
            },
        ));
    }

    out
}

enum RuleOrAttr {
    Rule(Span, Rule),
    Attr(Span, Attr),
    Error(syn::Error),
}

enum Attr {
    Alias(Ident),
    Dive,
    Skip,
}

impl Parse for RuleOrAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = Ident::parse_any(input)?;
        let span = ident.span();
        if ident == "rename" {
            <Token![=]>::parse(input)?;
            let value = <syn::LitStr as Parse>::parse(input)?.parse_with(Ident::parse_any)?;
            Ok(RuleOrAttr::Attr(span, Attr::Alias(value)))
        } else if ident == "dive" {
            Ok(RuleOrAttr::Attr(span, Attr::Dive))
        } else if ident == "skip" {
            Ok(RuleOrAttr::Attr(span, Attr::Skip))
        } else {
            Ok(Rule::parse_with_ident(input, ident)
                .map(|rule| RuleOrAttr::Rule(span, rule))
                .unwrap_or_else(RuleOrAttr::Error))
        }
    }
}

// TODO: macro to generate this boilerplate

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
    ByteLength {
        min: Option<usize>,
        max: Option<usize>,
    },
    Range {
        min: Option<Expr>,
        max: Option<Expr>,
    },
    Contains(String),
    Prefix(String),
    Suffix(String),
    Pattern(String),
    Custom(Expr),
}

struct RuleEmit {
    rule: TokenStream2,
    args: TokenStream2,
}

impl Rule {
    fn emit(&self, field: &Field) -> RuleEmit {
        let (rule, args) = match self {
            Rule::Ascii => (quote! {::garde::rules::ascii::apply}, quote! {()}),
            Rule::Alphanumeric => (quote! {::garde::rules::alphanumeric::apply}, quote! {()}),
            Rule::Email => (quote! {::garde::rules::email::apply}, quote! {()}),
            Rule::Url => (quote! {::garde::rules::url::apply}, quote! {()}),
            Rule::Ip => (
                quote! {::garde::rules::ip::apply},
                quote! {(::garde::rules::ip::IpKind::Any,)},
            ),
            Rule::IpV4 => (
                quote! {::garde::rules::ip::apply},
                quote! {(::garde::rules::ip::IpKind::V4,)},
            ),
            Rule::IpV6 => (
                quote! {::garde::rules::ip::apply},
                quote! {(::garde::rules::ip::IpKind::V6,)},
            ),
            Rule::CreditCard => (quote! {::garde::rules::credit_card::apply}, quote! {()}),
            Rule::PhoneNumber => (quote! {::garde::rules::phone_number::apply}, quote! {()}),
            Rule::Length { min, max } => {
                let (min, max) = (
                    min.unwrap_or(0),
                    max.map(|v| quote!(#v))
                        .unwrap_or_else(|| quote!(usize::MAX)),
                );
                (
                    quote! {::garde::rules::length::apply},
                    quote! {(#min, #max,)},
                )
            }
            Rule::ByteLength { min, max } => {
                let (min, max) = (
                    min.unwrap_or(0),
                    max.map(|v| quote!(#v))
                        .unwrap_or_else(|| quote!(usize::MAX)),
                );
                (
                    quote! {::garde::rules::byte_length::apply},
                    quote! {(#min, #max,)},
                )
            }
            Rule::Range { min, max } => {
                let ty = &field.ty;
                let (min, max) = (
                    min.as_ref()
                        .map(|v| v.to_token_stream())
                        .unwrap_or_else(|| quote!(<#ty as ::garde::rules::range::Bounds>::MIN)),
                    max.as_ref()
                        .map(|v| v.to_token_stream())
                        .unwrap_or_else(|| quote!(<#ty as ::garde::rules::range::Bounds>::MAX)),
                );
                (
                    quote! {::garde::rules::range::apply},
                    quote! {(&#min, &#max,)},
                )
            }
            Rule::Contains(s) => (quote! {::garde::rules::contains::apply}, quote! {(#s,)}),
            Rule::Prefix(s) => (quote! {::garde::rules::prefix::apply}, quote! {(#s,)}),
            Rule::Suffix(s) => (quote! {::garde::rules::suffix::apply}, quote! {(#s,)}),
            Rule::Pattern(s) => (
                quote! {::garde::rules::pattern::apply},
                quote! {{
                    static PATTERN: ::garde::rules::pattern::StaticPattern = ::garde::rules::pattern::init_pattern!(#s);
                    (&PATTERN,)
                }},
            ),
            Rule::Custom(e) => (quote! {#e}, quote! {&__garde_user_ctx}),
        };
        RuleEmit { rule, args }
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
            Rule::ByteLength { .. } => "byte_length",
            Rule::Range { .. } => "bounds",
            Rule::Contains(_) => "contains",
            Rule::Prefix(_) => "prefix",
            Rule::Suffix(_) => "suffix",
            Rule::Pattern(_) => "pattern",
            Rule::Custom(_) => "custom",
        }
    }
}

impl Rule {
    fn parse_with_ident(input: ParseStream, ident: Ident) -> syn::Result<Self> {
        macro_rules! parse_rule {
            ($ident:ident, $input:ident, $name:literal $body:block) => {
                if $ident == $name {
                    if $input.peek(syn::token::Paren) {
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
                    if $input.is_empty() || !$input.peek(syn::token::Paren) {
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
        parse_rule!(ident, input, "byte_length" (content) {
            parse_rule_byte_length(&content)?
        });
        parse_rule!(ident, input, "range" (content) {
            parse_rule_range(&content)?
        });
        parse_rule!(ident, input, "contains" (content) {
            parse_rule_contains(&content)?
        });
        parse_rule!(ident, input, "prefix" (content) {
            parse_rule_prefix(&content)?
        });
        parse_rule!(ident, input, "suffix" (content) {
            parse_rule_suffix(&content)?
        });
        parse_rule!(ident, input, "pattern" (content) {
            parse_rule_pattern(&content)?
        });
        parse_rule!(ident, input, "custom" (content) {
            parse_rule_custom(&content)?
        });

        Err(Error::new(ident.span(), "unrecognized validation rule"))
    }
}

/* impl Parse for Rule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = Ident::parse_any(input)?;
        Self::parse_with_ident(input, ident)
    }
}
 */
fn parse_rule_length(content: ParseStream) -> syn::Result<Rule> {
    let parts = content.parse_terminated(MetaNameValue::parse, Token![,])?;
    let mut min = None::<usize>;
    let mut max = None::<usize>;
    for part in parts.iter() {
        if part.path.is_ident("min") {
            if min.is_some() {
                return Err(Error::new(part.span(), "duplicate attribute"));
            }
            let value = parse_number_from_expr(&part.value)?;
            min = Some(value)
        } else if part.path.is_ident("max") {
            if max.is_some() {
                return Err(Error::new(part.span(), "duplicate attribute"));
            }
            let value = parse_number_from_expr(&part.value)?;
            max = Some(value)
        } else {
            return Err(Error::new(
                part.span(),
                format!("unexpected `{}`", part.path.to_token_stream()),
            ));
        }
    }
    match (min, max) {
        (Some(min), Some(max)) if min >= max => {
            return Err(Error::new(parts.span(), "min must be smaller than max"));
        }
        (None, None) => {
            return Err(Error::new(
                parts.span(),
                "please provide at least one of: `min`, `max`",
            ));
        }
        _ => {}
    }
    Ok(Rule::Length { min, max })
}

fn parse_rule_byte_length(content: ParseStream) -> syn::Result<Rule> {
    let parts = content.parse_terminated(MetaNameValue::parse, Token![,])?;
    let mut min = None::<usize>;
    let mut max = None::<usize>;
    for part in parts.iter() {
        if part.path.is_ident("min") {
            if min.is_some() {
                return Err(Error::new(part.span(), "duplicate attribute"));
            }
            let value = parse_number_from_expr(&part.value)?;
            min = Some(value)
        } else if part.path.is_ident("max") {
            if max.is_some() {
                return Err(Error::new(part.span(), "duplicate attribute"));
            }
            let value = parse_number_from_expr(&part.value)?;
            max = Some(value)
        } else {
            return Err(Error::new(
                part.span(),
                format!("unexpected `{}`", part.path.to_token_stream()),
            ));
        }
    }
    match (min, max) {
        (Some(min), Some(max)) if min >= max => {
            return Err(Error::new(parts.span(), "min must be smaller than max"));
        }
        (None, None) => {
            return Err(Error::new(
                parts.span(),
                "please provide at least one of: `min`, `max`",
            ));
        }
        _ => {}
    }
    Ok(Rule::ByteLength { min, max })
}

fn parse_number_from_expr<T>(expr: &Expr) -> syn::Result<T>
where
    T: FromStr,
    <T as FromStr>::Err: Display,
{
    let inner = match expr {
        Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Int(int),
            ..
        }) => int,
        _ => {
            return Err(Error::new(
                expr.span(),
                format!("value must be a valid `{}`", std::any::type_name::<T>()),
            ))
        }
    };
    let value = match inner.base10_parse::<T>() {
        Ok(value) => value,
        Err(error) => {
            return Err(Error::new(
                expr.span(),
                format!(
                    "value must be a valid `{}`: {error}",
                    std::any::type_name::<T>()
                ),
            ))
        }
    };
    Ok(value)
}

fn parse_rule_range(content: ParseStream) -> syn::Result<Rule> {
    let parts = content.parse_terminated(MetaNameValue::parse, Token![,])?;
    let mut min = None;
    let mut max = None;
    for part in parts.iter() {
        if part.path.is_ident("min") {
            if min.is_some() {
                return Err(Error::new(part.span(), "duplicate attribute"));
            }
            min = Some(part.value.clone())
        } else if part.path.is_ident("max") {
            if max.is_some() {
                return Err(Error::new(part.span(), "duplicate attribute"));
            }
            max = Some(part.value.clone())
        } else {
            return Err(Error::new(
                part.span(),
                format!("unexpected `{}`", part.path.to_token_stream()),
            ));
        }
    }
    if let (None, None) = (&min, &max) {
        return Err(Error::new(
            parts.span(),
            "please provide at least one of: `min`, `max`",
        ));
    }
    Ok(Rule::Range { min, max })
}
fn parse_rule_contains(content: ParseStream) -> syn::Result<Rule> {
    let content_span = content.span();
    let value = <syn::LitStr as Parse>::parse(content)?.value();
    if value.is_empty() {
        return Err(Error::new(content_span, "string must not be empty"));
    }
    Ok(Rule::Contains(value))
}
fn parse_rule_prefix(content: ParseStream) -> syn::Result<Rule> {
    let content_span = content.span();
    let value = <syn::LitStr as Parse>::parse(content)?.value();
    if value.is_empty() {
        return Err(Error::new(content_span, "string must not be empty"));
    }
    Ok(Rule::Prefix(value))
}
fn parse_rule_suffix(content: ParseStream) -> syn::Result<Rule> {
    let content_span = content.span();
    let value = <syn::LitStr as Parse>::parse(content)?.value();
    if value.is_empty() {
        return Err(Error::new(content_span, "string must not be empty"));
    }
    Ok(Rule::Suffix(value))
}
fn parse_rule_pattern(content: ParseStream) -> syn::Result<Rule> {
    let content_span = content.span();
    let value = <syn::LitStr as Parse>::parse(content)?.value();
    if value.is_empty() {
        return Err(Error::new(content_span, "string must not be empty"));
    }
    #[cfg(feature = "regex")]
    {
        if let Err(e) = regex::Regex::new(&value) {
            return Err(Error::new(content_span, format!("invalid regex: {e}")));
        }
    }
    Ok(Rule::Pattern(value))
}
fn parse_rule_custom(content: ParseStream) -> syn::Result<Rule> {
    let expr = syn::Expr::parse(content)?;
    match expr {
        Expr::Closure(_) | Expr::Path(_) => {}
        _ => {
            return Err(Error::new(
                expr.span(),
                "custom rule must be a closure or a path to a function",
            ))
        }
    }
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
