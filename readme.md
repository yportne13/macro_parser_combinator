# macro_parser_combinator

a parser library that looks like bnf

## usage

```
parser!{
    lit_temp: JsonValue = (float << whitespace) -> (JsonValue::Number)
        | (escaped_quoted << whitespace) -> (JsonValue::String)
        | "null" -> (|_| JsonValue::Null)
        | "true" -> (|_| JsonValue::Bool(true))
        | "false" -> (|_| JsonValue::Bool(false))
}
```

all the expression write in `parser!`. the above example will be expand to

```rust
pub fn lit_temp() -> Parser!(JsonValue) {
    xxxxxxx
}
```

`float`, `whitespace`, `escaped_quoted` is build in function. the string is normally use for match keyword. there is also a different type of string like `r".*"`, those string that start with `r` means that it is a regex expression.

* `>>`: for `a >> b`, parse a and b, but only return b. for example when a is keyword
* `<<`: for `a << b`, only return a
* `*`: return pair `(a,b)`
* `|`: return first match. a and b should be the same type
* `[`xxx`]`: try xxx, return `Option<xxx>`
* `{`xxx`}`: many xxx, return `Vec<xxx>`. if there is seperator, for example `,`, then use `{xxx(,)}`
* `-> (Fn)`: map
