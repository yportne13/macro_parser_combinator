use std::marker::PhantomData;
use std::ops::{Mul, Shr, Shl, BitOr};

#[derive(Copy, Clone)]
pub struct Parser<F: Copy, I, O>(pub F, pub PhantomData<I>, pub PhantomData<O>);

impl<F, I: Copy, O> Parser<F, I, O>
where
    F: Fn(I) -> (Option<O>, I) + Copy
{
    pub fn new(f: F) -> Self {
        Self(f, PhantomData::<I>, PhantomData::<O>)
    }
    pub fn run_with_out(&self, input: I) -> (Option<O>, I) {
        self.0(input)
    }
    pub fn run(&self, input: I) -> Option<O> {
        self.run_with_out(input).0
    }
    pub fn to_try(self) -> Parser<impl Fn(I) -> (Option<Option<O>>, I) + Copy, I, Option<O>> {
        let f = move |input: I| {
            let (ret, ret_input) = self.0(input);
            (Some(ret), ret_input)
        };
        Parser(f, std::marker::PhantomData::<I>, std::marker::PhantomData::<Option<O>>)
    }
    pub fn many(self) -> Parser<impl Fn(I) -> (Option<Vec<O>>, I) + Copy, I, Vec<O>> {
        let f = move |input: I| {
            let mut ret = Vec::new();
            let mut text = input;
            loop {
                let parse = self.0(text);
                match parse.0 {
                    Some(item) => {
                        ret.push(item);
                        text = parse.1;
                    },
                    None => break,
                }
            }
            (Some(ret), text)
        };
        Parser(f, std::marker::PhantomData::<I>, std::marker::PhantomData::<Vec<O>>)
    }
    pub fn many_sep<Fs>(self, sep: Fs) -> Parser<impl Fn(I) -> (Option<Vec<O>>, I) + Copy, I, Vec<O>>
    where
        Fs: Fn(I) -> Option<I> + Copy
    {
        let f = move |input: I| {
            let mut ret = Vec::new();
            let mut text = input;
            loop {
                let parse = self.0(text);
                match parse.0 {
                    Some(item) => {
                        ret.push(item);
                        let jump_sep = sep(parse.1);
                        match jump_sep {
                            Some(t) => {
                                text = t;
                            },
                            None => {
                                text = parse.1;
                                break
                            },
                        }
                    },
                    None => break,
                }
            }
            (Some(ret), text)
        };
        Parser(f, std::marker::PhantomData::<I>, std::marker::PhantomData::<Vec<O>>)
    }    
    pub fn map<M, X>(self, m: M) -> Parser<impl Fn(I) -> (Option<X>, I) + Copy, I, X>
    where
        M: Fn(O) -> X + Copy
    {
        let f = move |input: I| {
            let (ret, ret_input) = self.0(input);
            (ret.map(m), ret_input)
        };
        Parser(f, std::marker::PhantomData::<I>, std::marker::PhantomData::<X>)
    }
    pub fn and_then<M, X>(self, m: M) -> Parser<impl Fn(I) -> (Option<X>, I) + Copy, I, X>
    where
        M: Fn(O) -> Option<X> + Copy
    {
        let f = move |input: I| {
            let (ret, ret_input) = self.0(input);
            (ret.and_then(m), ret_input)
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
    F1: Fn(I) -> (Option<O1>, I),
    F2: Fn(I) -> (Option<O2>, I),
{
    type Output = Parser<impl Fn(I) -> (Option<(O1, O2)>, I) + Copy, I, (O1, O2)>;

    fn mul(self, rhs: Parser<F2, I, O2>) -> Self::Output {
        let f = move |input: I| {
            let (ol, next_input) = self.0(input);
            match ol {
                Some(ret_ol) => {
                    let (or, ret_input) = rhs.0(next_input);
                    match or {
                        Some(ret_or) => (Some((ret_ol, ret_or)), ret_input),
                        None => (None, input),
                    }
                },
                None => {
                    (None, input)
                },
            }
        };
        Parser(f, std::marker::PhantomData::<I>, std::marker::PhantomData::<(O1, O2)>)
    }
}

impl<F1: Copy, F2: Copy, I: Copy, O1, O2> Shr<Parser<F2, I, O2>> for Parser<F1, I, O1>
where
    F1: Fn(I) -> (Option<O1>, I),
    F2: Fn(I) -> (Option<O2>, I),
{
    type Output = Parser<impl Fn(I) -> (Option<O2>, I) + Copy, I, O2>;

    fn shr(self, rhs: Parser<F2, I, O2>) -> Self::Output {
        let f = move |input: I| {
            let (lefto, lefts) = self.0(input);
            match lefto {
                Some(_) => {
                    let (righto, rights) = rhs.0(lefts);
                    match righto {
                        Some(r) => (Some(r), rights),
                        None => (None, input)
                    }
                }
                None => (None, input)
            }
        };
        Parser(f, std::marker::PhantomData::<I>, std::marker::PhantomData::<O2>)
    }
}

impl<F1: Copy, F2: Copy, I: Copy, O1, O2> Shl<Parser<F2, I, O2>> for Parser<F1, I, O1>
where
    F1: Fn(I) -> (Option<O1>, I),
    F2: Fn(I) -> (Option<O2>, I),
{
    type Output = Parser<impl Fn(I) -> (Option<O1>, I) + Copy, I, O1>;

    fn shl(self, rhs: Parser<F2, I, O2>) -> Self::Output {
        let f = move |input: I| {
            let (lefto, lefts) = self.0(input);
            match lefto {
                Some(l) => {
                    let (righto, rights) = rhs.0(lefts);
                    match righto {
                        Some(_) => (Some(l), rights),
                        None => (None, input)
                    }
                }
                None => (None, input)
            }
        };
        Parser(f, std::marker::PhantomData::<I>, std::marker::PhantomData::<O1>)
    }
}

impl<F1: Copy, F2: Copy, I: Copy, O> BitOr<Parser<F2, I, O>> for Parser<F1, I, O>
where
    F1: Fn(I) -> (Option<O>, I),
    F2: Fn(I) -> (Option<O>, I),
{
    type Output = Parser<impl Fn(I) -> (Option<O>, I) + Copy, I, O>;

    fn bitor(self, rhs: Parser<F2, I, O>) -> Self::Output {
        let f = move |input: I| {
            let (lefto, lefts) = self.0(input);
            match lefto {
                Some(l) => (Some(l), lefts),
                None => {
                    let ret = rhs.0(input);
                    match ret.0 {
                        Some(r0) => (Some(r0), ret.1),
                        None => (
                            None,
                            ret.1)
                    }
                },
            }
        };
        Parser(f, std::marker::PhantomData::<I>, std::marker::PhantomData::<O>)
    }
}
