
extern crate proc_macro;
use macro_parser_combinator_core::*;
use proc_macro::TokenStream;

#[proc_macro]
pub fn bnf(input: TokenStream) -> TokenStream {
    let token =
        token!("\"") >> regex!(r"[a-zA-Z_]*").map(|x| format!("token!(\"{x}\")")) << token!("\"");
    let ismacro =
        regex!("[a-zA-Z_]*").map(|x| format!("{x}!()")) << whitespace!();
    let item =
        token | ismacro;
    let sig =
        token!("*") | token!(">>") | token!("<<") | token!("|");
    let expr =
        (item.clone() * sig * item.clone()).map(|x| format!("({}{}{})", x.0.0, x.0.1, x.1))
        | item;
    let bnf_ori =
        (regex!("[a-zA-Z_]*") << whitespace!() << token!(":") << token!("=")) * expr;// := -> : =
    let bnf = bnf_ori
        .map(|x| format!("macro_rules! {} {{
    () => {{
        {}
    }};
}}", x.0, x.1));
    let ret_string = bnf.run(&input.to_string()).unwrap();
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
