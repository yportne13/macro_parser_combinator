
use quote::{quote, ToTokens};
use syn::{Ident, parse::Parse, LitStr, parenthesized, bracketed};

use crate::expr::Expr;

pub enum Term {
    Func(Ident),
    Regex(LitStr),
    Token(LitStr),
    Paren(Box<Expr>),
    Many(Box<Expr>),
    ManySep(Box<Expr>, LitStr),
}

impl Parse for Term {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Ok(t) = input.parse::<TermParen>() {
            Ok(Term::Paren(Box::new(t.expr)))
        } else if let Ok(t) = input.parse::<TermMany>() {
            match t {
                TermMany::Many(e) => Ok(Term::Many(Box::new(e))),
                TermMany::ManySep(e, s) => Ok(Term::ManySep(Box::new(e), s)),
            }
            
        } else {
            input.clone().step(|cursor| {
                if let Some((lit, rest)) = cursor.literal() {
                    let repr = lit.clone().to_string().bytes().next();

                    match repr {
                        Some(b'"') => {
                            let str: LitStr = input.parse()?;
                            Ok((Term::Token(str), rest))
                        },
                        Some(b'r') => {
                            let str: LitStr = input.parse()?;
                            Ok((Term::Regex(str), rest))
                        },
                        _ => {
                            Err(cursor.error("expected literal"))
                        }
                    }
                }else if let Some((func, rest)) = cursor.ident() {
                    Ok((Term::Func(func), rest))
                }else {
                    Err(cursor.error("expected literal"))
                }
            })
        }
    }
}

impl ToTokens for Term {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        tokens.extend(match self {
            Term::Func(f) => {
                quote!(tobox!(#f()))
            },
            Term::Regex(str) => {
                quote!(regex!(#str))
            },
            Term::Token(str) => {
                quote!(token!(#str))
            },
            Term::Paren(expr) => {
                quote!((#expr))
            },
            Term::Many(expr) => {
                quote!((#expr).many())
            },
            Term::ManySep(expr, sep) => {
                quote!((#expr).many_sep(sep!(#sep)))
            }
        });
    }
}

struct TermParen {
    expr: Expr
}

impl Parse for TermParen {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        let _ = parenthesized!(content in input);
        let expr: Expr = content.parse()?;
        Ok(TermParen { expr })
    }
}

struct Sep {
    sep: LitStr,
}

impl Parse for Sep {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        let _ = parenthesized!(content in input);
        let sep = content.parse()?;
        Ok(Sep { sep })
    }
}

enum TermMany {
    Many(Expr),
    ManySep(Expr, LitStr),
}

impl Parse for TermMany {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        let _ = bracketed!(content in input);
        let expr: Expr = content.parse()?;//TODO:ManySep
        if let Ok(sep) = content.parse::<Sep>() {
            Ok(TermMany::ManySep(expr, sep.sep))
        }else {
            Ok(TermMany::Many(expr))
        }
    }
}
