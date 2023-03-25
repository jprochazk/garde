use std::collections::BTreeSet;
use std::str::FromStr;

use proc_macro::TokenStream;
use proc_macro2::{Ident, TokenStream as TokenStream2};
use quote::{format_ident, quote, ToTokens};
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::spanned::Spanned;
use syn::{Attribute, Data, DeriveInput, Error, Expr, Generics, MetaNameValue, Token, Type};

// TODO: test more error cases using `trybuild`
// TODO: if some rule feature is not enabled, it should `compile_error`
// TODO: custom error messages

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
        let context = &self.context;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        match &self.inner {
            InputKind::Struct(inner) => {
                let fields = inner.iter();

                quote! {
                    impl #impl_generics ::garde::Validate for #ident #ty_generics #where_clause {
                        type Context = #context;

                        fn validate(&self, ctx: &Self::Context) -> Result<(), ::garde::Errors> {
                            let mut errors = ::garde::Errors::new();

                            #(#fields)*

                            if !errors.fields.is_empty() {
                                return Err(errors);
                            }

                            Ok(())
                        }
                    }
                }
                .to_tokens(tokens)
            }
            InputKind::Enum(inner) => {
                let variants = inner.iter();

                quote! {
                    impl #impl_generics ::garde::Validate for #ident #ty_generics #where_clause {
                        type Context = #context;

                        fn validate(&self, ctx: &Self::Context) -> Result<(), ::garde::Errors> {
                            let mut errors = ::garde::Errors::new();

                            match self {
                                #(#variants)*
                            }

                            if !errors.fields.is_empty() {
                                return Err(errors);
                            }

                            Ok(())
                        }
                    }
                }
                .to_tokens(tokens)
            }
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
            let ty = match attr.parse_args_with(parse_context_meta) {
                Ok(ty) => ty,
                Err(e) => {
                    errors.push(e);
                    continue;
                }
            };
            inner = Some(ty);
        }
    }
    Context { inner }
}

fn parse_context_meta(input: ParseStream) -> syn::Result<Type> {
    let name = Ident::parse_any(input)?;
    if name != "context" {
        return Err(Error::new(name.span(), "unrecognized attribute"));
    }
    let content;
    syn::parenthesized!(content in input);
    content.parse::<Type>()
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
    kind: VariantKind,
}

enum VariantKind {
    Unit,
    Struct(Vec<Field>),
    Tuple(Vec<Field>),
}

impl ToTokens for VariantKind {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        match self {
            VariantKind::Unit => quote! {=> {}}.to_tokens(tokens),
            VariantKind::Struct(fields) => {
                let field_names = fields.iter().map(|field| field.ident.emit_ident());
                let fields = fields.iter().map(VariantField);
                quote!({#(#field_names)*} => {#(#fields)*}).to_tokens(tokens)
            }
            VariantKind::Tuple(fields) => {
                let field_names = fields.iter().map(|field| field.ident.emit_ident());
                let fields = fields.iter().map(VariantField);
                quote!((#(#field_names)*) => {#(#fields)*}).to_tokens(tokens)
            }
        }
    }
}

struct VariantField<'a>(&'a Field);

impl<'a> ToTokens for VariantField<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let rules = self
            .0
            .rules
            .iter()
            .map(|rule| rule.emit(self.0, &self.0.alias, false));

        quote! {
            #(
                #rules
            )*
        }
        .to_tokens(tokens)
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
            syn::Fields::Named(v) => {
                VariantKind::Struct(parse_fields(false, v.named.iter(), errors))
            }
            syn::Fields::Unnamed(v) => {
                VariantKind::Tuple(parse_fields(true, v.unnamed.iter(), errors))
            }
            syn::Fields::Unit => VariantKind::Unit,
        };
        out.push(Variant { ident, kind })
    }

    out
}

struct Field {
    ident: FieldIdent,
    ty: Type,
    alias: Option<Ident>,
    rules: BTreeSet<Rule>,
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let rules = self
            .rules
            .iter()
            .map(|rule| rule.emit(self, &self.alias, true));

        quote! {
            #(
                #rules
            )*
        }
        .to_tokens(tokens)
    }
}

enum FieldIdent {
    Name(Ident),
    Index(usize),
}

impl FieldIdent {
    fn emit_ident(&self) -> TokenStream2 {
        match self {
            FieldIdent::Name(v) => quote!(#v),
            FieldIdent::Index(v) => format_ident!("_{v}").to_token_stream(),
        }
    }

    fn emit_access(&self) -> TokenStream2 {
        match self {
            FieldIdent::Name(v) => quote!(#v),
            FieldIdent::Index(v) => TokenStream2::from_str(&format!("{v}")).unwrap(),
        }
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
            FieldIdent::Index(i)
        } else {
            FieldIdent::Name(f.ident.clone().unwrap())
        };

        let ty = f.ty.clone();
        let mut alias = None;
        let mut rules = BTreeSet::new();
        for attr in f.attrs.iter() {
            if attr.path().is_ident("garde") {
                let meta_list = match attr
                    .parse_args_with(Punctuated::<RuleOrAlias, Token![,]>::parse_terminated)
                {
                    Ok(rule) => rule,
                    Err(e) => {
                        errors.push(e);
                        continue;
                    }
                };

                for meta in meta_list {
                    match meta {
                        RuleOrAlias::Rule(rule) => {
                            if rules.contains(&rule) {
                                errors.push(Error::new(
                                    attr.meta.span(),
                                    format!("duplicate rule `{}`", rule.name()),
                                ));
                                continue;
                            }

                            rules.insert(rule);
                        }
                        RuleOrAlias::Alias(v) => {
                            if alias.is_some() {
                                errors.push(Error::new(
                                    attr.meta.span(),
                                    "duplicate attribute `rename`",
                                ));
                                continue;
                            }
                            alias = Some(v);
                        }
                    }
                }
            }
        }

        out.push(Field {
            ident,
            ty,
            alias,
            rules,
        })
    }

    out
}

enum RuleOrAlias {
    Rule(Rule),
    Alias(Ident),
}

impl Parse for RuleOrAlias {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = Ident::parse_any(input)?;
        if ident == "rename" {
            <Token![=]>::parse(input)?;
            let value = <syn::LitStr as Parse>::parse(input)?.parse_with(Ident::parse_any)?;
            Ok(RuleOrAlias::Alias(value))
        } else {
            Rule::parse_with_ident(input, ident).map(RuleOrAlias::Rule)
        }
    }
}

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

impl Rule {
    fn emit(&self, field: &Field, alias: &Option<Ident>, use_self: bool) -> TokenStream2 {
        let field_name = alias
            .as_ref()
            .map(|v| v.to_token_stream())
            .unwrap_or_else(|| field.ident.emit_ident());
        let field_access = if use_self {
            field.ident.emit_access()
        } else {
            field.ident.emit_ident()
        };
        let self_token = if use_self { Some(quote!(&self.)) } else { None };
        let check = match self {
            Rule::Ascii => {
                quote! {::garde::rules::ascii::apply(stringify!(#field_name), #self_token #field_access)}
            }
            Rule::Alphanumeric => {
                quote! {::garde::rules::alphanumeric::apply(stringify!(#field_name), #self_token #field_access)}
            }
            Rule::Email => {
                quote! {::garde::rules::email::apply(stringify!(#field_name), #self_token #field_access)}
            }
            Rule::Url => {
                quote! {::garde::rules::url::apply(stringify!(#field_name), #self_token #field_access)}
            }
            Rule::Ip => {
                quote! {::garde::rules::ip::apply(stringify!(#field_name), #self_token #field_access, ::garde::rules::ip::IpKind::Any)}
            }
            Rule::IpV4 => {
                quote! {::garde::rules::ip::apply(stringify!(#field_name), #self_token #field_access, ::garde::rules::ip::IpKind::V4)}
            }
            Rule::IpV6 => {
                quote! {::garde::rules::ip::apply(stringify!(#field_name), #self_token #field_access, ::garde::rules::ip::IpKind::V6)}
            }
            Rule::CreditCard => {
                quote! {::garde::rules::credit_card::apply(stringify!(#field_name), #self_token #field_access)}
            }
            Rule::PhoneNumber => {
                quote! {::garde::rules::phone_number::apply(stringify!(#field_name), #self_token #field_access)}
            }
            Rule::Length { min, max } => {
                let (min, max) = (
                    min.unwrap_or(0),
                    max.map(|v| quote!(#v))
                        .unwrap_or_else(|| quote!(usize::MAX)),
                );
                quote! {::garde::rules::length::apply(stringify!(#field_name), #self_token #field_access, #min, #max)}
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
                quote! {::garde::rules::range::apply(stringify!(#field_name), #self_token #field_access, &#min, &#max)}
            }
            Rule::Contains(s) => {
                quote! {::garde::rules::contains::apply(stringify!(#field_name), #self_token #field_access, #s)}
            }
            Rule::Prefix(s) => {
                quote! {::garde::rules::prefix::apply(stringify!(#field_name), #self_token #field_access, #s)}
            }
            Rule::Suffix(s) => {
                quote! {::garde::rules::suffix::apply(stringify!(#field_name), #self_token #field_access, #s)}
            }
            Rule::Pattern(s) => quote! {{
                static PATTERN: ::garde::rules::pattern::StaticPattern = ::garde::rules::pattern::init_pattern!(#s);
                ::garde::rules::pattern::apply(stringify!(#field_name), #self_token #field_access, &PATTERN)
            }},
            Rule::Custom(e) => quote! {{
                (#e)(stringify!(#field_name), #self_token #field_access, &ctx)
            }},
        };
        quote! {
            if let Err(e) = #check {
                errors.insert(stringify!(#field_name), e);
            }
        }
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

impl Parse for Rule {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = Ident::parse_any(input)?;
        Self::parse_with_ident(input, ident)
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
