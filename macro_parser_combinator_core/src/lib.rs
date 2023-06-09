#![feature(type_alias_impl_trait)]

extern crate lazy_static;

pub mod location;
pub mod parser;

pub use regex::Regex;
pub use lazy_static::lazy_static;

pub use crate::parser::Parser;

#[macro_export]
macro_rules! char {
    ($p: expr) => {
        {
            fn f(input: &str) -> (Option<&str>, &str) {
                if let Some(o) = input.strip_prefix($p) {
                    (Some(o), o)
                } else {
                    (
                        None,
                        input
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
            fn f(input: &str) -> (Option<&str>, &str) {
                if let Some(o) = input.strip_prefix($p) {
                    (Some($p), o)
                } else {
                    (
                        None,
                        input
                    )
                }
            }
            Parser(f, std::marker::PhantomData::<&str>, std::marker::PhantomData::<&str>)
        }
    };
}

#[macro_export]
macro_rules! split_token {
    ($p: expr) => {
        {
            fn f(input: &str) -> (Option<&str>, &str) {
                if let Some(o) = input.split_once($p) {
                    (Some(o.0), o.1)
                } else {
                    (
                        None,
                        input
                    )
                }
            }
            Parser(f, std::marker::PhantomData::<&str>, std::marker::PhantomData::<&str>)
        }
    };
}

#[macro_export]
macro_rules! token_throw {
    ($p: expr) => {
        {
            fn f(input: &str) -> (Option<()>, &str) {
                if let Some(o) = input.strip_prefix($p) {
                    (Some(()), o)
                } else {
                    (
                        None,
                        input
                    )
                }
            }
            Parser(f, std::marker::PhantomData::<&str>, std::marker::PhantomData::<()>)
        }
    };
}

#[macro_export]
macro_rules! whitespace {
    () => {
        //(token_base!(" ").map(|_| ())
        //    | token_base!("\n").map(|_| ())
        //    | token_base!("\r").map(|_| ())
        //    | token_base!("\t").map(|_| ())
        //).many().map(|_| "")
        //(token_throw!(" ")
        //    | token_throw!("\n")
        //    | token_throw!("\r")
        //    | token_throw!("\t")
        //).many().map(|_| "")
        //regex!(r"\s*").map(|_| "")
        {
            fn f(input: &str) -> (Option<&str>, &str) {
                let mut a = input.chars();
                let mut b = input.chars();
                loop {
                    match a.next() {
                        Some(' ') => {b.next();},
                        Some('\n') => {b.next();},
                        Some('\r') => {b.next();},
                        Some('\t') => {b.next();},
                        _ => {break;}
                    }
                }
                (Some(""), b.as_str())
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
            fn f(input: &str) -> (Option<String>, &str) {
                //let re = Regex::new($p).unwrap();
                lazy_static! {
                    static ref RE: Regex = Regex::new($p).unwrap();
                }
                let cap = RE.find(input).map(|x| x.as_str());
                let o = cap.and_then(|x| input.strip_prefix(x));
                match o {
                    Some(output) => {
                        (cap.map(|x| x.to_string()), output)
                    },
                    None => (None, input)
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
        //token_base!("\"") >> regex!(r#"(?:\\"|[^"])*"#) << token_base!("\"")
        token_base!("\"") >> split_token!("\"").map(|s| s.to_string())
    };
}

pub fn escaped_quoted<'a>() -> Parser!(String) {
    escaped_quoted!()
}

#[macro_export]
macro_rules! sep {
    ($p: expr) => {
        {
            fn f(input: &str) -> (Option<&str>) {
                if let Some(o) = input.strip_prefix($p) {
                    Some(o)
                } else {
                    (
                        None
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
            let f = |input| Box::new($p.0)(input);
            Parser::new(f)
        }
    };
}

#[macro_export]
macro_rules! Parser {
    ($t: tt) => {
        Parser<impl Fn(&'a str) -> (Option<$t>, &'a str) + Copy, &'a str, $t>
    };
    () => {
        Parser<impl Fn(&'a str) -> (Option<&'a str>, &'a str) + Copy, &'a str, &'a str>
    }
}
