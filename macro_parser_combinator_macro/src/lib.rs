#![feature(unboxed_closures)]
#![feature(type_alias_impl_trait)]

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, parse::Parse, parse_macro_input, Token, token::Eq, Type};

mod term;
use term::Term;
mod expr;



#[proc_macro]
pub fn term(input: TokenStream) -> TokenStream {
    let f = parse_macro_input!(input as Term);
    let ret = quote!(#f);
    ret.into()
}

struct Parser {
    name: Ident,
    out_type: Type,
    expr: expr::Expr,
}

impl Parse for Parser {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let out_type: Type = input.parse()?;
        //input.parse::<Token![::]>()?;
        input.parse::<Eq>()?;
        let expr: expr::Expr = input.parse()?;
        Ok(Self {
            name,
            out_type,
            expr,
        })
    }
}

#[proc_macro]
pub fn parser(input: TokenStream) -> TokenStream {
    let Parser { name, out_type, expr } = parse_macro_input!(input as Parser);
    let ret = quote!(pub fn #name<'a>() -> Parser!(#out_type) {
        #expr
    });
    ret.into()
}

#[test]
fn test() {

    let input = "abc";
    let parser = token!("ab") * token!("c");
    println!("{:?}", parser.run(input));
    let input = "abc";
    let parser = regex!(r"[a-zA-Z_]*");
    println!("{:?}", parser.run(input));
    let input = "abc   def";
    let parser = token!("abc") << token_base!("def");
    println!("{:?}", parser.run(input));
    let input = "data: 2992";
    let parser = token!("data:") >> int!();
    println!("{:?}", parser.run(input));
    let input = "data: 3.1415";
    let parser = token!("data:") >> float!();
    println!("{:?}", parser.run(input));
    let input = r#"a string: "abc""#;
    let parser = token!("a string:") >> escaped_quoted!();
    println!("{:?}", parser.run(input));

    let input = r#"abc := "abc""#;
    let token = token!("\"") >> regex!(r"[a-zA-Z_]*").map(|x| format!("token!(\"{x}\")")) << token!("\"");
    let parser =
        (regex!(r"[a-zA-Z_]*") << whitespace!() << token!(":=")) * token;
    println!("{:?}", parser.run(input));

}
