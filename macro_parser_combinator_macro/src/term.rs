
use quote::{quote, ToTokens};
use syn::{Ident, parse::Parse, LitStr, parenthesized, bracketed, braced, Token};

use crate::expr::Expr;

/// term
/// 1.function: whitespace
/// 2.regex: r"[a-z]*"
/// 3.token: "assign"
/// 4.paren: (a >> b)
/// 5.try: [a]
/// 6.many: {a}
/// 7.many1: {a+}
/// 8.many with sep: {a(",")}
/// 9.many with sep1: {a(",")+}
pub enum Term {
    Func(Ident),
    Regex(LitStr),
    Token(LitStr),
    Paren(Box<Expr>),
    Try(Box<Expr>),
    Many(Box<Expr>),
    Many1(Box<Expr>),
    ManySep(Box<Expr>, LitStr),
    ManySep1(Box<Expr>, LitStr),
}

impl Parse for Term {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if let Ok(t) = input.parse::<TermParen>() {
            Ok(Term::Paren(Box::new(t.expr)))
        } else if let Ok(t) = input.parse::<TermTry>() {
            Ok(Term::Try(Box::new(t.expr)))
        } else if let Ok(t) = input.parse::<TermMany>() {
            match t {
                TermMany::Many(e) => Ok(Term::Many(Box::new(e))),
                TermMany::Many1(e) => Ok(Term::Many1(Box::new(e))),
                TermMany::ManySep(e, s) => Ok(Term::ManySep(Box::new(e), s)),
                TermMany::ManySep1(e, s) => Ok(Term::ManySep1(Box::new(e), s)),
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
            Term::Try(expr) => {
                quote!((#expr).to_try())
            },
            Term::Many(expr) => {
                quote!((#expr).many())
            },
            Term::Many1(expr) => {
                quote!((#expr).many1())
            },
            Term::ManySep(expr, sep) => {
                quote!((#expr).many_sep(sep!(#sep)))
            },
            Term::ManySep1(expr, sep) => {
                quote!((#expr).many_sep1(sep!(#sep)))
            },
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

struct TermTry {
    expr: Expr
}

impl Parse for TermTry {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        let _ = bracketed!(content in input);
        let expr: Expr = content.parse()?;
        Ok(TermTry { expr })
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
    Many1(Expr),
    ManySep(Expr, LitStr),
    ManySep1(Expr, LitStr),
}

impl Parse for TermMany {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        let _ = braced!(content in input);
        let expr: Expr = content.parse()?;
        if let Ok(sep) = content.parse::<Sep>() {
            if let Ok(_) = content.parse::<Token![+]>() {
                Ok(TermMany::ManySep1(expr, sep.sep))
            }else {
                Ok(TermMany::ManySep(expr, sep.sep))
            }
        }else {
            if let Ok(_) = content.parse::<Token![+]>() {
                Ok(TermMany::Many1(expr))
            }else {
                Ok(TermMany::Many(expr))
            }
        }
    }
}
