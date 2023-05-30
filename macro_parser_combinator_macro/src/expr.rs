use quote::{ToTokens, quote};
use syn::{parse::Parse, Token, parenthesized, token::Eq};

use crate::term::Term;

/// first level of expr
/// 1. term -> (closure)
/// 2. term => (closure)
/// 3. term
pub enum Expr1 {
    Map(Term, syn::Expr),
    Flatmap(Term, syn::Expr),
    Term(Term),
}

impl Parse for Expr1 {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let term: Term = input.parse()?;
        if let Ok(_) = input.parse::<Token![->]>() {
            let content;
            let _ = parenthesized!(content in input);
            let expr = content.parse::<syn::Expr>()?;
            Ok(Expr1::Map(term, expr))
        } else if let Ok(_) = input.parse::<Eq>() {
            input.parse::<Token![>]>()?;
            let content;
            let _ = parenthesized!(content in input);
            let expr = content.parse::<syn::Expr>()?;
            Ok(Expr1::Flatmap(term, expr))
        }else {
            Ok(Expr1::Term(term))
        }
    }
}

impl ToTokens for Expr1 {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        tokens.extend(match self {
            Expr1::Map(t, e) => {
                quote!((#t).map(#e))
            },
            Expr1::Flatmap(t, e) => {
                quote!((#t).and_then(#e))
            },
            Expr1::Term(t) => quote!(#t),
        });
    }
}

/// second level of expr
/// 1. term * expr
/// 2. term << expr
/// 3. term >> expr
/// 4. term
pub enum Expr2 {
    Product(Expr1, Box<Expr>),
    Left(Expr1, Box<Expr>),
    Right(Expr1, Box<Expr>),
    Term(Expr1),
}

impl Parse for Expr2 {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let first: Expr1 = input.parse()?;
        if let Ok(_) = input.parse::<Token![*]>() {
            let second: Expr = input.parse()?;
            Ok(Expr2::Product(first, Box::new(second)))
        }else if let Ok(_) = input.parse::<Token![<<]>() {
            let second: Expr = input.parse()?;
            Ok(Expr2::Left(first, Box::new(second)))
        }else if let Ok(_) = input.parse::<Token![>>]>() {
            let second: Expr = input.parse()?;
            Ok(Expr2::Right(first, Box::new(second)))
        }else {
            Ok(Expr2::Term(first))
        }
    }
}

impl ToTokens for Expr2 {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        tokens.extend(match self {
            Expr2::Product(l, r) => quote!(#l * #r),
            Expr2::Left(l, r) => quote!(#l << #r),
            Expr2::Right(l, r) => quote!(#l >> #r),
            Expr2::Term(t) => quote!(#t),
        });
    }
}

/// third level of expr
/// 1. term | expr
/// 2. term
pub enum Expr {
    Or(Expr2, Box<Expr>),
    Term(Expr2),
}

impl Parse for Expr {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let first: Expr2 = input.parse()?;
        if let Ok(_) = input.parse::<Token![|]>() {
            let second: Expr = input.parse()?;
            Ok(Expr::Or(first, Box::new(second)))
        }else {
            Ok(Expr::Term(first))
        }
    }
}

impl ToTokens for Expr {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        tokens.extend(match self {
            Expr::Or(a, b) => quote!((#a) | #b),
            Expr::Term(a) => quote!(#a),
        });
    }
}
