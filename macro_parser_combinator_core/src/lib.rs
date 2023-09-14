//#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

extern crate lazy_static;

pub mod location;
//pub mod parser;
pub mod parser2;

pub use regex::Regex;
pub use lazy_static::lazy_static;

pub use crate::location::Location;
//pub use crate::parser::Parser;
pub use crate::parser2::Parser;

/*
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

/*pub fn escaped_quoted<'a>() -> Parser!(String) {
    escaped_quoted!()
}*/

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
*/

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
            fn fc(c: char) -> bool {
                $p.bytes().next().unwrap() == c as u8
            }
            fn f(input: &str) -> (Option<&str>, &str) {
                if let Some(o) = input.strip_prefix($p) {
                    (Some($p), o)
                } else {
                    (
                        None,
                        input,
                    )
                }
            }
            fn fl(input: &str, loc: Location) -> (Option<&str>, &str, Location) {
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
            Parser(fc, f, fl, std::marker::PhantomData::<char>, std::marker::PhantomData::<&str>, std::marker::PhantomData::<&str>)
        }
    };
}

#[macro_export]
macro_rules! token_throw {
    ($p: expr) => {
        {
            fn fc(c: char) -> bool {
                $p.bytes().next().unwrap() == c as u8
            }
            fn f(input: &str) -> (Option<()>, &str) {
                if let Some(o) = input.strip_prefix($p) {
                    (Some(()), o)
                } else {
                    (
                        None,
                        input,
                    )
                }
            }
            fn fl(input: &str, loc: Location) -> (Option<()>, &str, Location) {
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
            Parser(fc, f, fl, std::marker::PhantomData::<char>, std::marker::PhantomData::<&str>, std::marker::PhantomData::<()>)
        }
    };
}

#[macro_export]
macro_rules! split_token {
    ($p: expr) => {
        {
            fn fc(_: char) -> bool {
                true
            }
            fn f(input: &str) -> (Option<&str>, &str) {
                if let Some(o) = input.split_once($p) {
                    (Some(o.0), o.1)
                } else {
                    (
                        None,
                        input,
                    )
                }
            }
            fn fl(input: &str, loc: Location) -> (Option<&str>, &str, Location) {
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
            Parser(fc, f, fl, std::marker::PhantomData::<char>, std::marker::PhantomData::<&str>, std::marker::PhantomData::<&str>)
        }
    };
}

#[macro_export]
macro_rules! whitespace {
    () => {
        {
            fn fc(_: char) -> bool {
                true
            }
            fn f(input: &str) -> (Option<()>, &str) {
                let mut idx = 0;
                loop {
                    match input.bytes().nth(idx) {
                        Some(b' ') | Some(b'\t') => {idx += 1;}
                        Some(b'\n') | Some(b'\r') => {idx += 1;}
                        _ => {break;}
                    }
                }
                (Some(()), unsafe{input.get_unchecked(idx..)})
            }
            fn fl(input: &str, loc: Location) -> (Option<()>, &str, Location) {
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
            Parser(fc, f, fl, std::marker::PhantomData::<char>, std::marker::PhantomData::<&str>, std::marker::PhantomData::<()>)
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
macro_rules! sep {
    ($p: expr) => {
        token_throw!($p) << whitespace!()
    };
}

/*#[macro_export]
macro_rules! escaped_quoted {
    () => {
        //token_base!("\"").right(split_token!("\"").map(|s| s.to_string()))
        token_base!("\"") >> (split_token!("\"").map(|s| s.to_string()))
    };
}*/

pub fn escaped_quoted<'a>() -> Parser!(String) {
    //escaped_quoted!().map(|s| s.to_owned())
    token_base!("\"").right(split_token!("\"").map(|s| s.to_string()))
}

#[macro_export]
macro_rules! or_parser {
    ($t: ty, $($x: expr),*) => {{
        fn fcor(c: char) -> bool {
            $($x.0(c)||)* true
        }
        fn fp(i: &str) -> (Option<$t>, &str) {
            let first = i.bytes().next().unwrap() as char;
            $(
                if $x.0(first) {
                    let ret = $x.1(i);
                    if ret.0.is_some() {
                        return ret;
                    }
                }
            )*
            (None, i)
        }
        fn fl(i: &str, loc: Location) -> (Option<$t>, &str, Location) {
            let first = i.bytes().next().unwrap() as char;
            $(
                if $x.0(first) {
                    let ret = $x.2(i, loc);
                    if ret.0.is_some() {
                        return ret;
                    }
                }
            )*
            (None, i, loc)
        }
        Parser::new(fcor, fp, fl)
    }};
}

#[macro_export]
macro_rules! tobox {
    ($p: expr) => {
        {
            let fc = |c| Box::new($p.0)(c);
            let f = |input| Box::new($p.1)(input);
            let fl = |input, loc| Box::new($p.2)(input, loc);
            Parser::new(fc, f, fl)
        }
    };
}

#[macro_export]
macro_rules! Parser {
    ($t: ty) => {
        Parser<impl Fn(char) -> bool + Copy + 'a, impl Fn(&'a str) -> (Option<$t>, &'a str) + Copy, impl Fn(&'a str, Location) -> (Option<$t>, &'a str, Location) + Copy, char, &'a str, $t>
    };
    () => {
        Parser<impl Fn(char) -> bool + Copy, impl Fn(&'a str) -> (Option<&'a str>, &'a str) + Copy, char, &'a str, &'a str>
    }
}

/// float parser
pub fn float<'a>() -> Parser!(f64) {
    or_parser!(f64,
        my_float_inner(),
        (token_base!("nan").map(|_| f64::NAN)),
        (token_base!("inf").map(|_| f64::INFINITY)),
        (token_base!("infinity").map(|_| f64::INFINITY))
    )
}

pub fn my_float_inner<'a>() -> Parser!(f64) {
    fn fp(i: &str) -> (Option<f64>, &str) {
        let x = float_inner().1(i);
        x
    }
    //Parser::new(float_inner().0, fp)
    Parser::new(|c: char| c.is_ascii_digit() || c == '-' || c == '.' || c == '+',
    fp, float_inner().2)
}

fn float_inner<'a>() -> Parser!(f64) {
    ((token_base!("+").map(|_| 1) | token_base!("-").map(|_| -1)).to_try().map(|x| x.unwrap_or(1)) * (
        (digit1() * (token_base!(".") * digit1_size().to_try()).to_try()).map(|(i, x)| {
            (i as f64) + x.map(|(_, t)| t.map(|(a, b)| (a as f64)/(10_i32.pow(b as u32) as f64)).unwrap_or(0 as f64)).unwrap_or(0.0)
        })
        | (token_base!(".") * digit1_size()).map(|(_, (x, size))| (x as f64)/(10_i32.pow(size as u32) as f64))
    ) * (
        (token_base!("e") | token_base!("E")) *
        (token_base!("+").map(|_| true) | token_base!("-").map(|_| false)).to_try().map(|x| x.unwrap_or(true)) *
        digit1()
    ).map(|((_, sign), num)| if sign {
        10_i32.pow(num as u32) as f64
    } else {
        1.0/10_i32.pow(num as u32) as f64
    }).to_try().map(|x| x.unwrap_or(1.0))).map(|((sign, num), base)| (sign as f64) * num * base)
}

fn digit1<'a>() -> Parser!(u64) {
    let fc = |c: char| c.is_ascii_digit();
    fn fp(i: &str) -> (Option<u64>, &str) {
        let mut bytes = i.bytes();
        let mut offset = 0;
        let mut ret: u64 = 0;
        loop {
            let x = bytes.next().unwrap();
            if x.is_ascii_digit() {
                offset += 1;
                ret = ret * 10 + (x - b'0') as u64;
            } else {
                break;
            }
        }
        if offset == 0 {
            (None, i)
        } else {
            (Some(ret), unsafe{i.get_unchecked(offset..)})
        }
    }
    fn fl(i: &str, loc: Location) -> (Option<u64>, &str, Location) {
        let mut bytes = i.bytes();
        let mut offset = 0;
        let mut ret: u64 = 0;
        let mut loc = loc;
        loop {
            let x = bytes.next().unwrap();
            if x.is_ascii_digit() {
                offset += 1;
                ret = ret * 10 + (x - b'0') as u64;
            } else {
                break;
            }
        }
        if offset == 0 {
            (None, i, loc)
        } else {
            loc.offset += offset;
            loc.col += offset;
            (Some(ret), unsafe{i.get_unchecked(offset..)}, loc)
        }
    }
    Parser::new(fc, fp, fl)
}

fn digit1_size<'a>() -> Parser!((u64, u64)) {
    let fc = |c: char| c.is_ascii_digit();
    fn fp(i: &str) -> (Option<(u64, u64)>, &str) {
        let mut bytes = i.bytes();
        let mut offset = 0;
        let mut ret: u64 = 0;
        loop {
            let x = bytes.next().unwrap();
            if x.is_ascii_digit() {
                offset += 1;
                ret = ret * 10 + (x - b'0') as u64;
            } else {
                break;
            }
        }
        if offset == 0 {
            (None, i)
        } else {
            (Some((ret, offset as u64)), unsafe{i.get_unchecked(offset..)})
        }
    }
    fn fl(i: &str, loc: Location) -> (Option<(u64, u64)>, &str, Location) {
        let mut bytes = i.bytes();
        let mut offset = 0;
        let mut ret: u64 = 0;
        let mut loc = loc;
        loop {
            let x = bytes.next().unwrap();
            if x.is_ascii_digit() {
                offset += 1;
                ret = ret * 10 + (x - b'0') as u64;
            } else {
                break;
            }
        }
        if offset == 0 {
            (None, i, loc)
        } else {
            loc.offset += offset;
            loc.col += offset;
            (Some((ret, offset as u64)), unsafe{i.get_unchecked(offset..)}, loc)
        }
    }
    Parser::new(fc, fp, fl)
}
