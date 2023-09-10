use std::marker::PhantomData;
use std::ops::{Mul, Shr, Shl, BitOr};

#[derive(Copy, Clone)]
pub struct Parser<Ff: Copy, Fp: Copy, Ic: Copy, Is: Copy, O>(
    pub Ff,
    pub Fp,
    pub PhantomData<Ic>,
    pub PhantomData<Is>,
    pub PhantomData<O>
);

macro_rules! ParserType {
    ($t: ty) => {
        Parser<impl Fn(Ic) -> bool + Copy, impl Fn(Is) -> (Option<$t>, Is) + Copy, Ic, Is, $t>
    };
}

impl<Ff, Fp, Ic: Copy, Is: Copy, O> Parser<Ff, Fp, Ic, Is, O>
where
    Ff: Fn(Ic) -> bool + Copy,
    Fp: Fn(Is) -> (Option<O>, Is) + Copy
{
    pub fn new(ff: Ff, fp: Fp) -> Self {
        Self(ff, fp, PhantomData::<Ic>, PhantomData::<Is>,PhantomData::<O>)
    }
    pub fn first_match(&self, c: Ic) -> bool {
        self.0(c)
    }
    pub fn run_with_out(&self, input: Is) -> (Option<O>, Is) {
        self.1(input)
    }
    pub fn run(&self, input: Is) -> Option<O> {
        self.run_with_out(input).0
    }
    pub fn to_try(self) -> ParserType!(Option<O>) {
        let ff = |_| true;
        let fp = move |input: Is| {
            let (ret, ret_input) = self.1(input);
            (Some(ret), ret_input)
        };
        Parser(ff, fp, PhantomData::<Ic>, PhantomData::<Is>, PhantomData::<Option<O>>)
    }
    pub fn many(self) -> ParserType!(Vec<O>) {
        let ff = |_| true;
        let fp = move |input: Is| {
            let mut ret = Vec::with_capacity(2);
            let mut text = input;
            loop {
                let parse = self.1(text);
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
        Parser(ff, fp, PhantomData::<Ic>, PhantomData::<Is>, PhantomData::<Vec<O>>)
    }
    pub fn many1(self) -> ParserType!(Vec<O>) {
        let fc = move |c| self.0(c);
        let fp = move |input: Is| {
            let mut ret = Vec::with_capacity(2);
            let mut text = input;
            loop {
                let parse = self.1(text);
                match parse.0 {
                    Some(item) => {
                        ret.push(item);
                        text = parse.1;
                    },
                    None => break,
                }
            }
            if ret.is_empty() {
                (None, input)
            }else {
                (Some(ret), text)
            }
        };
        Parser(fc, fp, PhantomData::<Ic>, PhantomData::<Is>, PhantomData::<Vec<O>>)
    }
    pub fn many_sep<Fcs, Fps, Os>(self, sep: Parser<Fcs, Fps, Ic, Is, Os>) -> ParserType!(Vec<O>)
    where
        Fcs: Fn(Ic) -> bool + Copy,
        Fps: Fn(Is) -> (Option<Os>, Is) + Copy
    {
        let fc = |_| true;
        let fp = move |input: Is| {
            let mut ret = Vec::with_capacity(2);
            let mut text = input;
            loop {
                let parse = self.1(text);
                match parse.0 {
                    Some(item) => {
                        ret.push(item);
                        let jump_sep = sep.1(parse.1);
                        match jump_sep.0 {
                            Some(_) => {
                                text = jump_sep.1;
                            },
                            None => {
                                text = jump_sep.1;
                                break
                            },
                        }
                    },
                    None => break,
                }
            }
            (Some(ret), text)
        };
        Parser(fc, fp, PhantomData::<Ic>, PhantomData::<Is>, PhantomData::<Vec<O>>)
    }    
    pub fn many_sep1<Fcs, Fps, Os>(self, sep: Parser<Fcs, Fps, Ic, Is, Os>) -> ParserType!(Vec<O>)
    where
        Fcs: Fn(Ic) -> bool + Copy,
        Fps: Fn(Is) -> (Option<Os>, Is) + Copy
    {
        let fc = move |c| self.0(c);
        let fp = move |input: Is| {
            let mut ret = Vec::with_capacity(2);
            let mut text = input;
            loop {
                let parse = self.1(text);
                match parse.0 {
                    Some(item) => {
                        ret.push(item);
                        let jump_sep = sep.1(parse.1);
                        match jump_sep.0 {
                            Some(_) => {
                                text = jump_sep.1;
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
            if ret.is_empty() {
                (None, input)
            }else {
                (Some(ret), text)
            }
        };
        Parser(fc, fp, PhantomData::<Ic>, PhantomData::<Is>, PhantomData::<Vec<O>>)
    }
    pub fn map<M, X>(self, m: M) -> ParserType!(X)
    where
        M: Fn(O) -> X + Copy
    {
        let fp = move |input: Is| {
            let (ret, ret_input) = self.1(input);
            (ret.map(m), ret_input)
        };
        Parser(self.0, fp, PhantomData::<Ic>, PhantomData::<Is>, PhantomData::<X>)
    }
    pub fn and_then<M, X>(self, m: M) -> ParserType!(X)
    where
        M: Fn(O) -> Option<X> + Copy
    {
        let fp = move |input: Is| {
            let (ret, ret_input) = self.1(input);
            (ret.and_then(m), ret_input)
        };
        Parser(self.0, fp, PhantomData::<Ic>, PhantomData::<Is>, PhantomData::<X>)
    }

    pub fn right<Fcr, Fsr, Or>(self, rhs: Parser<Fcr, Fsr, Ic, Is, Or>) -> ParserType!(Or)
    where
        Fcr: Fn(Ic) -> bool + Copy,
        Fsr: Fn(Is) -> (Option<Or>, Is) + Copy,
    {
        let f = move |input: Is| {
            let (lefto, lefts) = self.1(input);
            match lefto {
                Some(_) => {
                    let (righto, rights) = rhs.1(lefts);
                    match righto {
                        Some(r) => (Some(r), rights),
                        None => (None, input)
                    }
                }
                None => (None, input)
            }
        };
        Parser(self.0, f, PhantomData::<Ic>, PhantomData::<Is>, PhantomData::<Or>)
    }
}

impl<Fc1: Copy, Fs1: Copy, Fc2: Copy, Fs2: Copy, Ic: Copy, Is: Copy, O1, O2> Mul<Parser<Fc2, Fs2, Ic, Is, O2>> for Parser<Fc1, Fs1, Ic, Is, O1>
where
    Fc1: Fn(Ic) -> bool,
    Fc2: Fn(Ic) -> bool,
    Fs1: Fn(Is) -> (Option<O1>, Is),
    Fs2: Fn(Is) -> (Option<O2>, Is),
{
    type Output = ParserType!((O1, O2));

    fn mul(self, rhs: Parser<Fc2, Fs2, Ic, Is, O2>) -> Self::Output {
        let f = move |input: Is| {
            let (ol, next_input) = self.1(input);
            match ol {
                Some(ret_ol) => {
                    let (or, ret_input) = rhs.1(next_input);
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
        Parser(self.0, f, PhantomData::<Ic>, PhantomData::<Is>, PhantomData::<(O1, O2)>)
    }
}

impl<Fc1, Fs1, Fc2, Fs2, Ic: Copy, Is: Copy, O1, O2> Shr<Parser<Fc2, Fs2, Ic, Is, O2>> for Parser<Fc1, Fs1, Ic, Is, O1>
where
    Fc1: Fn(Ic) -> bool + Copy,
    Fc2: Fn(Ic) -> bool + Copy,
    Fs1: Fn(Is) -> (Option<O1>, Is) + Copy,
    Fs2: Fn(Is) -> (Option<O2>, Is) + Copy,
{
    type Output = ParserType!(O2);

    fn shr(self, rhs: Parser<Fc2, Fs2, Ic, Is, O2>) -> Self::Output {
        let f = move |input: Is| {
            let (lefto, lefts) = self.1(input);
            match lefto {
                Some(_) => {
                    let (righto, rights) = rhs.1(lefts);
                    match righto {
                        Some(r) => (Some(r), rights),
                        None => (None, input)
                    }
                }
                None => (None, input)
            }
        };
        Parser(self.0, f, PhantomData::<Ic>, PhantomData::<Is>, PhantomData::<O2>)
    }
}

impl<Fc1: Copy, Fs1: Copy, Fc2: Copy, Fs2: Copy, Ic: Copy, Is: Copy, O1, O2> Shl<Parser<Fc2, Fs2, Ic, Is, O2>> for Parser<Fc1, Fs1, Ic, Is, O1>
where
    Fc1: Fn(Ic) -> bool,
    Fc2: Fn(Ic) -> bool,
    Fs1: Fn(Is) -> (Option<O1>, Is),
    Fs2: Fn(Is) -> (Option<O2>, Is),
{
    type Output = ParserType!(O1);

    fn shl(self, rhs: Parser<Fc2, Fs2, Ic, Is, O2>) -> Self::Output {
        let f = move |input: Is| {
            let (lefto, lefts) = self.1(input);
            match lefto {
                Some(l) => {
                    let (righto, rights) = rhs.1(lefts);
                    match righto {
                        Some(_) => (Some(l), rights),
                        None => (None, input)
                    }
                }
                None => (None, input)
            }
        };
        Parser(self.0, f, PhantomData::<Ic>, PhantomData::<Is>, PhantomData::<O1>)
    }
}

impl<Fc1: Copy, Fs1: Copy, Fc2: Copy, Fs2: Copy, Ic: Copy, Is: Copy, O> BitOr<Parser<Fc2, Fs2, Ic, Is, O>> for Parser<Fc1, Fs1, Ic, Is, O>
where
    Fc1: Fn(Ic) -> bool,
    Fc2: Fn(Ic) -> bool,
    Fs1: Fn(Is) -> (Option<O>, Is),
    Fs2: Fn(Is) -> (Option<O>, Is),
{
    type Output = ParserType!(O);

    fn bitor(self, rhs: Parser<Fc2, Fs2, Ic, Is, O>) -> Self::Output {
        let fc = move |c| self.0(c) || rhs.0(c);
        let fs = move |input: Is| {
            let (lefto, lefts) = self.1(input);
            match lefto {
                Some(l) => (Some(l), lefts),
                None => {
                    let ret = rhs.1(input);
                    match ret.0 {
                        Some(r0) => (Some(r0), ret.1),
                        None => (
                            None,
                            ret.1)
                    }
                },
            }
        };
        Parser(fc, fs, PhantomData::<Ic>, PhantomData::<Is>, PhantomData::<O>)
    }
}
