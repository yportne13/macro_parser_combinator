use macro_parser_combinator::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum JsonValue {
    Null,
    String(String),
    Number(f64),
    Bool(bool),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>)
}

parser!{
    lit_temp: JsonValue = (float << whitespace) -> (JsonValue::Number)
        | (escaped_quoted << whitespace) -> (JsonValue::String)
        | "null" -> (|_| JsonValue::Null)
        | "true" -> (|_| JsonValue::Bool(true))
        | "false" -> (|_| JsonValue::Bool(false))

    lit: JsonValue = whitespace >> lit_temp

    array: JsonValue = (whitespace >> "[") >>
        [value(",")] -> (JsonValue::Array) << "]"

    value: JsonValue = lit | array | obj

    key_value: (String, JsonValue) = whitespace >> (escaped_quoted << whitespace << ":") * value

    obj: JsonValue = whitespace >> "{" >>
        [key_value(",")] -> (|x| JsonValue::Object(x.into_iter().collect::<HashMap<String, JsonValue>>()))
        << "}"
}

fn main() {
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
    println!("{:?}", obj().run(input));
}
