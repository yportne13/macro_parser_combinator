
pub use macro_parser_combinator_core::*;
pub use macro_parser_combinator_macro::*;
pub use regex::Regex;

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use macro_parser_combinator_core::*;

    #[derive(Debug)]
    enum Json {
        Null,
        String(String),
        Number(f64),
        Bool(bool),
        Array(Vec<Json>),
        Object(BTreeMap<String, Json>)
    }

    fn lit_temp<'a>() -> Parser!(Json) {
        token!("null").map(|_| Json::Null)
            | (float!() << whitespace!()).map(Json::Number)
            | (escaped_quoted!() <<whitespace!()).map(Json::String)
            | token!("true").map(|_| Json::Bool(true))
            | token!("false").map(|_| Json::Bool(false))
    }

    fn lit<'a>() -> Parser!(Json) {
        whitespace!() >> lit_temp()
    }

    fn array<'a>() -> Parser!(Json) {
        (whitespace!() >> token!("[")) >>
            tobox!(value()).many_sep(sep!(",")).map(Json::Array)//TODO: lit => value
            << (whitespace!() >> token!("]"))
    }

    fn value<'a>() -> Parser!(Json) {
        lit() | array() | tobox!(obj())
    }

    fn key_value<'a>() -> Parser!((String, Json)) {
        whitespace!() >> ((escaped_quoted!() << whitespace!() << token!(":")) *
            value())
    }

    fn obj<'a>() -> Parser!(Json) {
        whitespace!() >> token!("{") >> (
            key_value().many_sep(sep!(",")).map(|x| Json::Object(x.into_iter().collect::<BTreeMap<String, Json>>()))
        ) << (whitespace!() >> token!("}"))
    }

    #[test]
    fn test() {

        let input = r#"
{
  "Company name" : "Microsoft Corporation",
  "Ticker"  : "MSFT",
  "Active"  : true,
  "Price"   : 30.66,
  "Shares outstanding" : 8.38e9,
  "Related companies" : [ "HPQ", "IBM", "YHOO", "DELL", "GOOG" ]
}
"#;
        let obj = obj();
        let ret = obj.run_with_out(input);
        println!("mem size: {}", std::mem::size_of_val(&obj));

        let x = r#"Some(Object({"Active": Bool(true), "Company name": String("Microsoft Corporation"), "Price": Number(30.66), "Related companies": Array([String("HPQ"), String("IBM"), String("YHOO"), String("DELL"), String("GOOG")]), "Shares outstanding": Number(8380000000.0), "Ticker": String("MSFT")}))"#;
        println!("{:?}", ret.0);
        let ret_str = format!("{:?}", ret.0);
        if ret_str.eq(x) {
            println!("✔️ : is correct");
        }else {
            println!("❌ : not correct")
        }

    }
}
