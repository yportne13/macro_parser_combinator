use std::marker::PhantomData;
use std::ops::{Mul, Shr, Shl, BitOr};
use crate::location::Location;

#[derive(Copy, Clone)]
pub struct Parser<F: Copy, I, O>(pub F, pub PhantomData<I>, pub PhantomData<O>);

impl<F, I: Copy, O> Parser<F, I, O>
where
    F: Fn(I, Location) -> (Option<O>, I, Location) + Copy
{
    pub fn new(f: F) -> Self {
        Self(f, PhantomData::<I>, PhantomData::<O>)
    }
    pub fn run_with_out(&self, input: I, loc: Location) -> (Option<O>, I, Location) {
        self.0(input, loc)
    }
    pub fn run(&self, input: I) -> Option<O> {
        self.run_with_out(input, Location::new()).0
    }
    pub fn many(self) -> Parser<impl Fn(I, Location) -> (Option<Vec<O>>, I, Location) + Copy, I, Vec<O>> {
        let f = move |input: I, loc: Location| {
            let mut ret = Vec::new();
            let mut text = input;
            let mut loc_parse = loc;
            loop {
                let parse = self.0(text, loc_parse);
                match parse.0 {
                    Some(item) => {
                        ret.push(item);
                        text = parse.1;
                        loc_parse = parse.2;
                    },
                    None => break,
                }
            }
            (Some(ret), text, loc_parse)
        };
        Parser(f, std::marker::PhantomData::<I>, std::marker::PhantomData::<Vec<O>>)
    }
    pub fn many_sep<Fs>(self, sep: Fs) -> Parser<impl Fn(I, Location) -> (Option<Vec<O>>, I, Location) + Copy, I, Vec<O>>
    where
        Fs: Fn(I, Location) -> (Option<I>, Location) + Copy
    {
        let f = move |input: I, loc: Location| {
            let mut ret = Vec::new();
            let mut text = input;
            let mut loc_parse = loc;
            loop {
                let parse = self.0(text, loc_parse);
                match parse.0 {
                    Some(item) => {
                        ret.push(item);
                        let jump_sep = sep(parse.1, parse.2);
                        match jump_sep.0 {
                            Some(t) => {
                                text = t;
                                loc_parse = jump_sep.1;
                            },
                            None => {
                                text = parse.1;
                                loc_parse = parse.2;
                                break
                            },
                        }
                    },
                    None => break,
                }
            }
            (Some(ret), text, loc_parse)
        };
        Parser(f, std::marker::PhantomData::<I>, std::marker::PhantomData::<Vec<O>>)
    }    
    pub fn map<M, X>(self, m: M) -> Parser<impl Fn(I, Location) -> (Option<X>, I, Location) + Copy, I, X>
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

/*pub fn y_combinator<F1, F2, I, O>(f: &dyn Fn(Parser<F1, I, O>) -> Parser<F2, I, O>)
    -> Parser<impl Fn(I, Location) -> (Option<O>, I, Location) + Copy, I, O>
where
    F1: Fn(I, Location) -> (Option<O>, I, Location) + Copy,
    F2: Fn(I, Location) -> (Option<O>, I, Location) + Copy,
{
    
}*/

impl<F1: Copy, F2: Copy, I: Copy, O1, O2> Mul<Parser<F2, I, O2>> for Parser<F1, I, O1>
where
    F1: Fn(I, Location) -> (Option<O1>, I, Location),
    F2: Fn(I, Location) -> (Option<O2>, I, Location),
{
    type Output = Parser<impl Fn(I, Location) -> (Option<(O1, O2)>, I, Location) + Copy, I, (O1, O2)>;

    fn mul(self, rhs: Parser<F2, I, O2>) -> Self::Output {
        let f = move |input: I, loc: Location| {
            let (ol, next_input, next_loc) = self.0(input, loc);
            match ol {
                Some(ret_ol) => {
                    let (or, ret_input, ret_loc) = rhs.0(next_input, next_loc);
                    match or {
                        Some(ret_or) => (Some((ret_ol, ret_or)), ret_input, ret_loc),
                        None => (None, input, loc),
                    }
                },
                None => {
                    (None, input, loc)
                },
            }
        };
        Parser(f, std::marker::PhantomData::<I>, std::marker::PhantomData::<(O1, O2)>)
    }
}

impl<F1: Copy, F2: Copy, I: Copy, O1, O2> Shr<Parser<F2, I, O2>> for Parser<F1, I, O1>
where
    F1: Fn(I, Location) -> (Option<O1>, I, Location),
    F2: Fn(I, Location) -> (Option<O2>, I, Location),
{
    type Output = Parser<impl Fn(I, Location) -> (Option<O2>, I, Location) + Copy, I, O2>;

    fn shr(self, rhs: Parser<F2, I, O2>) -> Self::Output {
        let f = move |input: I, loc: Location| {
            let (lefto, lefts, loc_left) = self.0(input, loc);
            match lefto {
                Some(_) => {
                    let (righto, rights, loc_right) = rhs.0(lefts, loc_left);
                    match righto {
                        Some(r) => (Some(r), rights, loc_right),
                        None => (None, input, loc)
                    }
                }
                None => (None, input, loc)
            }
        };
        Parser(f, std::marker::PhantomData::<I>, std::marker::PhantomData::<O2>)
    }
}

impl<F1: Copy, F2: Copy, I: Copy, O1, O2> Shl<Parser<F2, I, O2>> for Parser<F1, I, O1>
where
    F1: Fn(I, Location) -> (Option<O1>, I, Location),
    F2: Fn(I, Location) -> (Option<O2>, I, Location),
{
    type Output = Parser<impl Fn(I, Location) -> (Option<O1>, I, Location) + Copy, I, O1>;

    fn shl(self, rhs: Parser<F2, I, O2>) -> Self::Output {
        let f = move |input: I, loc: Location| {
            let (lefto, lefts, loc_left) = self.0(input, loc);
            match lefto {
                Some(l) => {
                    let (righto, rights, loc_right) = rhs.0(lefts, loc_left);
                    match righto {
                        Some(_) => (Some(l), rights, loc_right),
                        None => (None, input, loc)
                    }
                }
                None => (None, input, loc)
            }
        };
        Parser(f, std::marker::PhantomData::<I>, std::marker::PhantomData::<O1>)
    }
}

impl<F1: Copy, F2: Copy, I: Copy, O> BitOr<Parser<F2, I, O>> for Parser<F1, I, O>
where
    F1: Fn(I, Location) -> (Option<O>, I, Location),
    F2: Fn(I, Location) -> (Option<O>, I, Location),
{
    type Output = Parser<impl Fn(I, Location) -> (Option<O>, I, Location) + Copy, I, O>;

    fn bitor(self, rhs: Parser<F2, I, O>) -> Self::Output {
        let f = move |input: I, loc: Location| {
            let (lefto, lefts, loc_left) = self.0(input, loc);
            match lefto {
                Some(l) => (Some(l), lefts, loc_left),
                None => {
                    let ret = rhs.0(input, loc_left);
                    match ret.0 {
                        Some(r0) => (Some(r0), ret.1, ret.2),
                        None => (
                            None,
                            ret.1, ret.2)
                    }
                },
            }
        };
        Parser(f, std::marker::PhantomData::<I>, std::marker::PhantomData::<O>)
    }
}
