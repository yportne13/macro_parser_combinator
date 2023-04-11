#![feature(unboxed_closures)]
#![feature(type_alias_impl_trait)]

extern crate proc_macro;
use macro_parser_combinator_core::*;
use proc_macro::TokenStream;

//type PF<I, O> = impl Fn(I, Location) -> (Result<O, (String, Location)>, I, O);
//fn regex() -> Parser<impl Fn, &str, &str> {
//    token!(r#"r""#) >> regex!(r#"[^"]*"#).map(|x| format!(r#"regex!(r"{x}")"#)) << token!("\"")
//}

fn bnf_parser(input: TokenStream) -> String {
    let regex =
        token!(r#"r""#) >> regex!(r#"[^"]*"#).map(|x| format!(r#"regex!(r"{x}")"#)) << token!("\"");
    let token =
        token!("\"") >> regex!(r#"[^"]*"#).map(|x| format!("token!(\"{x}\")")) << token!("\"");
    let ismacro =
        regex!("[a-zA-Z_]+").map(|x| format!("{x}!()")) << whitespace!();
    let single_item =
        regex | token | ismacro;
    let many_sep =
        token!("[") >> (single_item.clone()
            * (token!("(") >> regex!(r"[^)]+") << token!(")")))
            .map(|x| format!("(whitespace!() >> {}).many_sep(sep!(\"{}\"))", x.0, x.1)) << token!("]");//TODO:use many_sep()
    let many_nosep =
        token!("[") >> single_item.clone().map(|x| format!("{x}.many()")) << token!("]");
    let many_item =
        many_sep | many_nosep;
    let item =
        many_item | single_item;
    let sig =
        token!("*") | token!(">>") | token!("<<") | token!("|");
    let expr =
        (item.clone() * sig * item.clone()).map(|x| format!("(({}){}({}))", x.0.0, x.0.1, x.1))
        | item;
    let bnf_ori =
        (regex!("[a-zA-Z_]*") << whitespace!() << token!(":") << token!("=")) * expr;// := -> : =
    let bnf = bnf_ori
        .map(|x| format!("macro_rules! {} {{
    () => {{
        {}
    }};
}}", x.0, x.1));
    let input = input.to_string();
    let ret_string = bnf.run(&input).unwrap();
    ret_string
}

#[proc_macro]
pub fn bnf_string(input: TokenStream) -> TokenStream {
    let ret_string = bnf_parser(input);
    format!("fn s() -> String {{ r#\"{}\"#.to_string() }}", ret_string).parse().unwrap()
    //ret_string.parse().unwrap()
}

#[proc_macro]
pub fn bnf(input: TokenStream) -> TokenStream {
    let ret_string = bnf_parser(input);
    //"fn answer() -> u32 { 42 }".parse().unwrap()
    //format!("fn s() -> String {{ r#\"{}\"#.to_string() }}", ret_string).parse().unwrap()
    ret_string.parse().unwrap()
}
/*
#[proc_macro]
pub fn bnf(input: TokenStream) -> TokenStream {
    format!("fn s() -> String {{ r#\"{}\"#.to_string() }}", input).parse().unwrap()
}*/

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
