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
            fn f(input: &str, loc: Location) -> (Option<&str>, &str, Location) {
                if let Some(o) = input.strip_prefix($p) {
                    let loc_parse = loc.update_char($p);
                    (Some(o), o, loc_parse.0)
                } else {
                    (
                        None,
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
            fn f(input: &str, loc: Location) -> (Option<&str>, &str, Location) {
                if let Some(o) = input.strip_prefix($p) {
                    let loc_parse = loc.update($p);
                    (Some($p), o, loc_parse.0)
                } else {
                    (
                        None,
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
macro_rules! split_token {
    ($p: expr) => {
        {
            fn f(input: &str, loc: Location) -> (Option<&str>, &str, Location) {
                if let Some(o) = input.split_once($p) {
                    let loc_parse = loc.update($p);
                    (Some(o.0), o.1, loc_parse.0)
                } else {
                    (
                        None,
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
macro_rules! token_throw {
    ($p: expr) => {
        {
            fn f(input: &str, loc: Location) -> (Option<()>, &str, Location) {
                if let Some(o) = input.strip_prefix($p) {
                    let loc_parse = loc.update($p);
                    (Some(()), o, loc_parse.0)
                } else {
                    (
                        None,
                        input,
                        loc
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
        {
            fn f(input: &str, loc: Location) -> (Option<()>, &str, Location) {
                let mut idx = 0;
                let mut loc = loc;
                loop {
                    match input.bytes().nth(idx) {
                        Some(b' ') | Some(b'\t') => {idx += 1;loc.col += 1;loc.offset += 1;}
                        Some(b'\n') | Some(b'\r') => {idx += 1;loc.col += 1;loc.offset += 1;loc.line += 1;}
                        _ => {break;}
                    }
                }
                (Some(()), unsafe{input.get_unchecked(idx..)}, loc)
            }
            Parser(f, std::marker::PhantomData::<&str>, std::marker::PhantomData::<()>)
        }
    };
}

pub fn whitespace<'a>() -> Parser!(()) {
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
            fn f(input: &str, loc: Location) -> (Option<String>, &str, Location) {
                //let re = Regex::new($p).unwrap();
                lazy_static! {
                    static ref RE: Regex = Regex::new($p).unwrap();
                }
                let cap = RE.find(input).map(|x| x.as_str());
                let o = cap.and_then(|x| input.strip_prefix(x));
                match o {
                    Some(output) => {
                        let loc_parse = loc.update(cap.unwrap());
                        (cap.map(|x| x.to_string()), output, loc_parse.0)
                    },
                    None => (None, input, loc)
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
            fn f(input: &str, loc: Location) -> (Option<&str>, Location) {
                if let Some(o) = input.strip_prefix($p) {
                    let loc_parse = loc.update($p);
                    let mut idx = 0;
                    let mut loc = loc_parse.0;
                    loop {
                        match o.bytes().nth(idx) {
                            Some(b' ') | Some(b'\t') => {idx += 1;loc.col += 1;loc.offset += 1;}
                            Some(b'\n') | Some(b'\r') => {idx += 1;loc.col += 1;loc.offset += 1;loc.line += 1;}
                            _ => {break;}
                        }
                    }
                    (Some(unsafe{o.get_unchecked(idx..)}), loc_parse.0)
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
    ($t: ty) => {
        Parser<impl Fn(&'a str, Location) -> (Option<$t>, &'a str, Location) + Copy, &'a str, $t>
    };
    () => {
        Parser<impl Fn(&'a str, Location) -> (Option<&'a str>, &'a str, Location) + Copy, &'a str, &'a str>
    }
}
