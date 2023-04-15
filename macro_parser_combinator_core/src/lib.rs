#![feature(type_alias_impl_trait)]

extern crate lazy_static;

pub mod location;
pub mod parser;

pub use regex::Regex;
pub use lazy_static::lazy_static;

pub use crate::location::Location;
pub use crate::parser::Parser;

#[macro_export]
macro_rules! char {
    ($p: expr) => {
        {
            fn f(input: &str, loc: Location) -> (Result<&str, (String, Location)>, &str, Location) {
                if let Some(o) = input.strip_prefix($p) {
                    let loc_parse = loc.update_char($p);
                    (Ok(o), o, loc_parse.0)
                } else {
                    (
                        Err((format!("should be char {} but get {}",
                            $p,
                            input.get(0..1).unwrap_or("")), loc)),
                        input,
                        loc
                    )
                }
            }
            Parser(f, std::marker::PhantomData::<&str>, std::marker::PhantomData::<&str>)
        }
    };
}

#[macro_export]
macro_rules! token_base {
    ($p: expr) => {
        {
            fn f(input: &str, loc: Location) -> (Result<&str, (String, Location)>, &str, Location) {
                if let Some(o) = input.strip_prefix($p) {
                    let loc_parse = loc.update($p);
                    (Ok($p), o, loc_parse.0)
                } else {
                    (
                        Err((format!("should be token {} but get {}",
                            $p,
                            input.get(0..($p.len())).unwrap_or("")), loc)),
                        input,
                        loc
                    )
                }
            }
            Parser(f, std::marker::PhantomData::<&str>, std::marker::PhantomData::<&str>)
        }
    };
}

#[macro_export]
macro_rules! whitespace {
    () => {
        //(token_base!(" ")
        //    | token_base!("\n")
        //    | token_base!("\r")
        //    | token_base!("\t")
        //).many()
        //regex!(r"\s*")
        {
            fn f(input: &str, loc: Location) -> (Result<&str, (String, Location)>, &str, Location) {
                let mut a = input.chars();
                let mut b = input.chars();
                let mut loc = loc;
                loop {
                    match a.next() {
                        Some(' ') => {b.next();loc.col += 1;},
                        Some('\n') => {b.next();loc.col = 1;loc.line += 1;},
                        Some('\r') => {b.next();loc.col = 1;loc.line += 1;},
                        Some('\t') => {b.next();loc.col += 1;},
                        _ => {break;}
                    }
                }
                (Ok(""), b.as_str(), loc)
            }
            Parser(f, std::marker::PhantomData::<&str>, std::marker::PhantomData::<&str>)
        }
    };
}

pub fn whitespace<'a>() -> Parser!() {
    whitespace!()
}

#[macro_export]
macro_rules! token {
    ($p: expr) => {
        token_base!($p) << whitespace!()
    };
}

#[macro_export]
macro_rules! regex {
    ($p: expr) => {
        {
            fn f(input: &str, loc: Location) -> (Result<String, (String, Location)>, &str, Location) {
                //let re = Regex::new($p).unwrap();
                lazy_static! {
                    static ref RE: Regex = Regex::new($p).unwrap();
                }
                let cap = RE.find(input).map(|x| x.as_str());
                let o = cap.and_then(|x| input.strip_prefix(x));
                match o {
                    Some(output) => {
                        let loc_parse = loc.update(cap.unwrap());
                        (cap.map(|x| x.to_string()).ok_or(("regex error".to_string(), loc_parse.0)), output, loc_parse.0)
                    },
                    None => (Err((format!("should be regex {}", $p), loc)), input, loc)
                }
            }
            Parser(f, std::marker::PhantomData::<&str>, std::marker::PhantomData::<String>)
        }
    };
}

#[macro_export]
macro_rules! int {
    () => {
        regex!(r"[-+]?[0-9]+").map(|x| x.parse::<i64>().unwrap())
    };
}

pub fn int<'a>() -> Parser!(i64) {
    regex!(r"[-+]?[0-9]+").map(|x| x.parse::<i64>().unwrap())
}

#[macro_export]
macro_rules! float {
    () => {
        regex!(r"[-+]?([0-9]*\.)?[0-9]+([eE][-+]?[0-9]+)?").map(|x| x.parse::<f64>().unwrap())
    };
}

pub fn float<'a>() -> Parser!(f64) {
    float!()
}

#[macro_export]
macro_rules! escaped_quoted {
    () => {
        token_base!("\"") >> regex!(r#"(?:\\"|[^"])*"#) << token_base!("\"")
    };
}

pub fn escaped_quoted<'a>() -> Parser!(String) {
    escaped_quoted!()
}

#[macro_export]
macro_rules! sep {
    ($p: expr) => {
        {
            fn f(input: &str, loc: Location) -> (Option<&str>, Location) {
                if let Some(o) = input.strip_prefix($p) {
                    let loc_parse = loc.update($p);
                    (Some(o), loc_parse.0)
                } else {
                    (
                        None,
                        loc
                    )
                }
            }
            f
        }
    };
}

#[macro_export]
macro_rules! tobox {
    ($p: expr) => {
        {
            let f = |input, loc: Location| Box::new($p.0)(input, loc);
            Parser::new(f)
        }
    };
}

#[macro_export]
macro_rules! Parser {
    ($t: tt) => {
        Parser<impl Fn(&'a str, Location) -> (Result<$t, (String, Location)>, &'a str, Location) + Copy, &'a str, $t>
    };
    () => {
        Parser<impl Fn(&'a str, Location) -> (Result<&'a str, (String, Location)>, &'a str, Location) + Copy, &'a str, &'a str>
    }
}
