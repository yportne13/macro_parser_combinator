use std::marker::PhantomData;
use std::ops::{Mul, Shr, Shl, BitOr};
use crate::location::Location;

#[derive(Copy, Clone)]
pub struct Parser<F: Copy, I, O>(pub F, pub PhantomData<I>, pub PhantomData<O>);

impl<F, I: Copy, O> Parser<F, I, O>
where
    F: Fn(I, Location) -> (Result<O, (String, Location)>, I, Location) + Copy
{
    pub fn run_with_out(&self, input: I, loc: Location) -> (Result<O, (String, Location)>, I, Location) {
        self.0(input, loc)
    }
    pub fn run(&self, input: I) -> Result<O, (String, Location)> {
        self.run_with_out(input, Location::new()).0
    }
    pub fn many(self) -> Parser<impl Fn(I, Location) -> (Result<Vec<O>, (String, Location)>, I, Location) + Copy, I, Vec<O>> {
        let f = move |input: I, loc: Location| {
            let mut ret = Vec::new();
            let mut text = input;
            let mut loc_parse = loc;
            loop {
                let parse = self.0(text, loc_parse);
                match parse.0 {
                    Ok(item) => {
                        ret.push(item);
                        text = parse.1;
                        loc_parse = parse.2;
                    },
                    Err(_) => break,
                }
            }
            (Ok(ret), text, loc_parse)
        };
        Parser(f, std::marker::PhantomData::<I>, std::marker::PhantomData::<Vec<O>>)
    }
    pub fn map<M, X>(self, m: M) -> Parser<impl Fn(I, Location) -> (Result<X, (String, Location)>, I, Location) + Copy, I, X>
    where
        M: Fn(O) -> X + Copy
    {
        let f = move |input: I, loc: Location| {
            let (ret, ret_input, ret_loc) = self.0(input, loc);
            (ret.map(m), ret_input, ret_loc)
        };
        Parser(f, std::marker::PhantomData::<I>, std::marker::PhantomData::<X>)
    }
}

impl<F1: Copy, F2: Copy, I: Copy, O1, O2> Mul<Parser<F2, I, O2>> for Parser<F1, I, O1>
where
    F1: Fn(I, Location) -> (Result<O1, (String, Location)>, I, Location),
    F2: Fn(I, Location) -> (Result<O2, (String, Location)>, I, Location),
{
    type Output = Parser<impl Fn(I, Location) -> (Result<(O1, O2), (String, Location)>, I, Location) + Copy, I, (O1, O2)>;

    fn mul(self, rhs: Parser<F2, I, O2>) -> Self::Output {
        let f = move |input: I, loc: Location| {
            let (ol, next_input, next_loc) = self.0(input, loc);
            match ol {
                Ok(ret_ol) => {
                    let (or, ret_input, ret_loc) = rhs.0(next_input, next_loc);
                    match or {
                        Ok(ret_or) => (Ok((ret_ol, ret_or)), ret_input, ret_loc),
                        Err(l) => (Err(l), input, loc),
                    }
                },
                Err(l) => {
                    (Err(l), input, loc)
                },
            }
        };
        Parser(f, std::marker::PhantomData::<I>, std::marker::PhantomData::<(O1, O2)>)
    }
}

impl<F1: Copy, F2: Copy, I: Copy, O1, O2> Shr<Parser<F2, I, O2>> for Parser<F1, I, O1>
where
    F1: Fn(I, Location) -> (Result<O1, (String, Location)>, I, Location),
    F2: Fn(I, Location) -> (Result<O2, (String, Location)>, I, Location),
{
    type Output = Parser<impl Fn(I, Location) -> (Result<O2, (String, Location)>, I, Location) + Copy, I, O2>;

    fn shr(self, rhs: Parser<F2, I, O2>) -> Self::Output {
        let f = move |input: I, loc: Location| {
            let (lefto, lefts, loc_left) = self.0(input, loc);
            match lefto {
                Ok(_) => {
                    let (righto, rights, loc_right) = rhs.0(lefts, loc_left);
                    match righto {
                        Ok(r) => (Ok(r), rights, loc_right),
                        Err(e) => (Err(e), input, loc)
                    }
                }
                Err(e) => (Err(e), input, loc)
            }
        };
        Parser(f, std::marker::PhantomData::<I>, std::marker::PhantomData::<O2>)
    }
}

impl<F1: Copy, F2: Copy, I: Copy, O1, O2> Shl<Parser<F2, I, O2>> for Parser<F1, I, O1>
where
    F1: Fn(I, Location) -> (Result<O1, (String, Location)>, I, Location),
    F2: Fn(I, Location) -> (Result<O2, (String, Location)>, I, Location),
{
    type Output = Parser<impl Fn(I, Location) -> (Result<O1, (String, Location)>, I, Location) + Copy, I, O1>;

    fn shl(self, rhs: Parser<F2, I, O2>) -> Self::Output {
        let f = move |input: I, loc: Location| {
            let (lefto, lefts, loc_left) = self.0(input, loc);
            match lefto {
                Ok(l) => {
                    let (righto, rights, loc_right) = rhs.0(lefts, loc_left);
                    match righto {
                        Ok(_) => (Ok(l), rights, loc_right),
                        Err(e) => (Err(e), input, loc)
                    }
                }
                Err(e) => (Err(e), input, loc)
            }
        };
        Parser(f, std::marker::PhantomData::<I>, std::marker::PhantomData::<O1>)
    }
}

impl<F1: Copy, F2: Copy, I: Copy, O> BitOr<Parser<F2, I, O>> for Parser<F1, I, O>
where
    F1: Fn(I, Location) -> (Result<O, (String, Location)>, I, Location),
    F2: Fn(I, Location) -> (Result<O, (String, Location)>, I, Location),
{
    type Output = Parser<impl Fn(I, Location) -> (Result<O, (String, Location)>, I, Location) + Copy, I, O>;

    fn bitor(self, rhs: Parser<F2, I, O>) -> Self::Output {
        let f = move |input: I, loc: Location| {
            let (lefto, lefts, loc_left) = self.0(input, loc);
            match lefto {
                Ok(l) => (Ok(l), lefts, loc_left),
                Err(left_err) => {
                    let ret = rhs.0(input, loc_left);
                    match ret.0 {
                        Ok(r0) => (Ok(r0), ret.1, ret.2),
                        Err(right_err) => (
                            Err((format!("{} or {}", left_err.0, right_err.0), right_err.1)),
                            ret.1, ret.2)
                    }
                },
            }
        };
        Parser(f, std::marker::PhantomData::<I>, std::marker::PhantomData::<O>)
    }
}
