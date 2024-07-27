mod check;
mod emit;
mod model;
mod syntax;
mod util;

use proc_macro::{Delimiter, Literal, Span, TokenStream, TokenTree};
use proc_macro_error::proc_macro_error;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_error]
#[proc_macro_derive(Validate, attributes(garde))]
pub fn derive_validate(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as DeriveInput);
    let input = match syntax::parse(input) {
        Ok(v) => v,
        Err(e) => return e.into_compile_error().into(),
    };
    let input = match check::check(input) {
        Ok(v) => v,
        Err(e) => return e.into_compile_error().into(),
    };
    emit::emit(input).into()
}

#[proc_macro]
pub fn select(input: TokenStream) -> TokenStream {
    fn parse_literal_digits_only(lit: Literal) -> syn::Result<String> {
        let span = lit.span();
        let lit = lit.to_string();
        if lit.chars().any(|v| !v.is_ascii_digit()) {
            return Err(syn::Error::new(span.into(), "unexpected token"));
        }
        Ok(lit)
    }

    fn expect_ident_or_digit(
        tokens: &mut impl Iterator<Item = TokenTree>,
    ) -> syn::Result<(Span, String)> {
        let Some(tt) = tokens.next() else {
            return Err(syn::Error::new(
                Span::call_site().into(),
                "incomplete input",
            ));
        };
        let span = tt.span();
        match tt {
            TokenTree::Ident(ident) => Ok((span, ident.to_string())),
            TokenTree::Literal(lit) => parse_literal_digits_only(lit).map(|lit| (span, lit)),
            _ => Err(syn::Error::new(span.into(), "unexpected token")),
        }
    }

    fn expect_punct(tokens: &mut impl Iterator<Item = TokenTree>, punct: char) -> syn::Result<()> {
        let Some(tt) = tokens.next() else {
            return Err(syn::Error::new(
                Span::call_site().into(),
                format!("expected `{punct}`"),
            ));
        };
        let span = tt.span();
        let TokenTree::Punct(ident) = tt else {
            return Err(syn::Error::new(span.into(), format!("expected `{punct}`")));
        };
        if ident.as_char() != punct {
            return Err(syn::Error::new(span.into(), format!("expected `{punct}`")));
        }
        Ok(())
    }

    fn expect_end(tokens: &mut impl Iterator<Item = TokenTree>) -> syn::Result<()> {
        match tokens.next() {
            Some(tt) => Err(syn::Error::new(tt.span().into(), "expected end of input")),
            None => Ok(()),
        }
    }

    fn parse_report_ident(
        tokens: &mut impl Iterator<Item = TokenTree>,
    ) -> syn::Result<(Span, String)> {
        let ident = expect_ident_or_digit(tokens)?;
        expect_punct(tokens, ',')?;
        Ok(ident)
    }

    fn parse_components(tokens: &mut impl Iterator<Item = TokenTree>) -> syn::Result<Vec<String>> {
        let mut idents = vec![];
        while let Some(tt) = tokens.next() {
            match tt {
                TokenTree::Group(group) if group.delimiter() == Delimiter::Bracket => {
                    let mut inner = group.stream().into_iter();
                    idents.push(expect_ident_or_digit(&mut inner)?.1.to_string());
                    expect_end(&mut inner)?;
                }
                TokenTree::Literal(lit) if idents.is_empty() => {
                    idents.push(parse_literal_digits_only(lit)?)
                }
                TokenTree::Ident(ident) if idents.is_empty() => idents.push(ident.to_string()),
                TokenTree::Punct(punct) if punct.as_char() == '.' => {
                    idents.push(expect_ident_or_digit(tokens)?.1.to_string());
                }
                other => return Err(syn::Error::new(other.span().into(), "unexpected token")),
            }
        }

        Ok(idents)
    }

    let mut tokens = input.into_iter();
    let report_ident: proc_macro2::Ident = match parse_report_ident(&mut tokens) {
        Ok((span, report_ident)) => proc_macro2::Ident::new(&report_ident.to_string(), span.into()),
        Err(e) => return e.into_compile_error().into(),
    };
    let components = match parse_components(&mut tokens) {
        Ok(components) => components,
        Err(e) => return e.into_compile_error().into(),
    };

    quote! {{
        let report = &#report_ident;
        let needle = [#(#components),*];
        report.iter()
            .filter(move |(path, _)| {
                if needle.len() > path.len() {
                    return false
                }
                let mut path = path.__iter().rev().map(|(_, v)| v.as_str());
                for left in needle.iter().copied() {
                    match path.next() {
                        Some(right) => if left != right { return false },
                        None => return false,
                    }
                }
                true
            })
            .map(|(_, error)| error)
    }}
    .into()
}
