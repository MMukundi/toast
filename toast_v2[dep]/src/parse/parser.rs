use crate::or::Or;
use std::{convert::Infallible, fmt::Debug, marker::PhantomData, mem::MaybeUninit, str::FromStr, f32::consts::E};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParserOutput<'s, T> {
    pub output: T,
    pub remaining: &'s str,
    pub chars_consumed: usize,
}
impl <'s, T> ParserOutput<'s,T>{
    #[inline]
    pub fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> ParserOutput<'s,U> {
        ParserOutput { output: (f)(self.output), remaining: self.remaining, chars_consumed: self.chars_consumed }
    } 
    #[inline]
    pub fn with_offset(mut self, offset: usize) -> Self {
        self.chars_consumed+=offset;
        self
    }
}

mod sealed {
    pub(super) trait ParserOutput {}
    pub(super) trait ParserError {}
}

impl <T> sealed::ParserError for WithLocation<T>{}
impl sealed::ParserError for Infallible{}
impl sealed::ParserError for (){}
impl <L:sealed::ParserError,R:sealed::ParserError> sealed::ParserError for Or<L,R>{}

impl <T> sealed::ParserOutput for ParserOutput<'_, T>{}
impl sealed::ParserOutput for Infallible{}
impl sealed::ParserOutput for (){}
impl <L:sealed::ParserOutput,R:sealed::ParserOutput> sealed::ParserOutput for Or<L,R>{}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WithLocation<T> {
    pub error: T,
    pub error_location: usize,
}
impl<T> WithLocation<T> {
    #[inline]
    pub fn new(error_location: usize, error: T) -> Self {
        Self { error_location, error }
    }
    #[inline]
    pub fn map<U, F: FnMut(T) -> U>(self, mut f: F) -> WithLocation<U> {
        WithLocation {
            error_location: self.error_location,
            error: (f)(self.error),
        }
    }
    #[inline]
    pub fn with_offset(self, offset: usize) -> Self {
        Self {
            error_location: self.error_location + offset,
            error: self.error,
        }
    }
    // #[inline]
    // pub fn offset(&mut self, offset: usize) {
    //     self.index += offset;
    // }
}

pub trait Parser<'s> {
    type Output;
    type Error;
    fn parse_next(
        &mut self,
        source: &'s str,
    ) -> Result<ParserOutput<'s, Self::Output>, Self::Error>;
    fn or<P: Parser<'s>>(self, right: P) -> OrParser<Self, P>
    where
        Self: Sized,
    {
        OrParser::new(self, right)
    }    
    fn map<U,F:FnMut(Self::Output)->U>(self, map: F) -> MapParser<Self, F>
    where
        Self: Sized,
    {
        MapParser::new(self, map)
    }  
    fn map_err<U,F:FnMut(Self::Error)->U>(self, map_err: F) -> MapErrParser<Self, F>
    where
        Self: Sized,
    {
        MapErrParser::new(self, map_err)
    }
    fn map_both<U,V,F:FnMut(Self::Output)->U,G:FnMut(Self::Error)->V>(self, map: F,map_err: G) -> MapBothParser<Self, F,G>
    where
        Self: Sized,
    {
        MapBothParser::new(self, map,map_err)
    }
    fn and<P: Parser<'s>>(self, right: P) -> AndParser<Self, P>
    where
        Self: Sized,
    {
        AndParser::new(self, right)
    }
    fn many<C>(self) -> ManyParser<Self, C>
    where
        Self: Sized,
    {
        ManyParser::new(self)
    }
    fn at_least_one<C>(self) -> AtLeastOneParser<Self, C>
    where
        Self: Sized,
    {
        AtLeastOneParser::new(self)
    }
    fn many_with<C>(self,intital:C) -> ManyWithParser<Self, C>
    where
        Self: Sized,
    {
        ManyWithParser::new(self,intital)
    }
    fn at_least_one_with<C>(self,intital:C) -> AtLeastOneWithParser<Self, C>
    where
        Self: Sized,
    {
        AtLeastOneWithParser::new(self,intital)
    }
    fn extend_with_many<C>(self,collection:&mut C) -> ExtendWithManyParser<Self, C>
    where
        Self: Sized,
    {
        ExtendWithManyParser::new(self,collection)
    }
    fn extend_with_at_least_one<C>(self,collection:&mut C) -> ExtendWithAtLeastOneParser<Self, C>
    where
        Self: Sized,
    {
        ExtendWithAtLeastOneParser::new(self,collection)
    }
}

impl <'s> Parser<'s> for char {
    type Output = ();
    type Error = Option<char>;
    fn parse_next(
        &mut self,
        source: &'s str,
    ) -> Result<ParserOutput<'s, Self::Output>, Self::Error> {
        let mut chars = source.chars();
        let first_char = chars.next();
        if first_char == Some(*self) {
            Ok(ParserOutput {
                output: (),
                remaining: chars.as_str(),
                chars_consumed: 1,
            })
        } else {
            Err(first_char)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstCharParser<const C:char>;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstCharError<const C:char>(pub Option<char>);
impl <const C:char> Parser<'_> for ConstCharParser<C>{
    type Output =();

    type Error = ConstCharError<C>;

    fn parse_next<'s>(
        &mut self,
        source: &'s str,
    ) -> Result<ParserOutput<'s, Self::Output>, Self::Error> {
        let mut chars = source.chars();
        let first_char = chars.next();
        if first_char == Some(C) {
            Ok(ParserOutput {
                output: (),
                remaining: chars.as_str(),
                chars_consumed: 1,
            })
        } else {
            Err(ConstCharError(first_char))
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrParserErr {
    LengthMismatch {
        min_expected_length: usize,
        source_length: usize,
    },
    CharMismatch {
        index: usize,
        expected: char,
        actual: char,
    },
}
impl <'s> Parser<'s> for &str {
    type Output = &'s str;
    type Error = WithLocation<StrParserErr>;
    fn parse_next(
        &mut self,
        source: &'s str,
    ) -> Result<ParserOutput<'s, Self::Output>, Self::Error> {
        if source.len() < self.len() {
            Err(WithLocation::new(
                source.len(),
                StrParserErr::LengthMismatch {
                    min_expected_length: self.len(),
                    source_length: source.len(),
                },
            ))
        } else {
            let source_chars = source.chars();
            let expected_chars = self.chars();
            let misoutput_char =
                expected_chars
                    .zip(source_chars)
                    .enumerate()
                    .find_map(|(i, (expected, source))| {
                        (expected != source).then_some((i, expected, source))
                    });
            if let Some((index, expected, actual)) = misoutput_char {
                Err(WithLocation::new(
                    index,
                    StrParserErr::CharMismatch {
                        index,
                        expected,
                        actual,
                    },
                ))
            } else {
                let len = self.len();
                Ok(ParserOutput {
                    output: &source[..len],
                    remaining: &source[len..],
                    chars_consumed: len,
                })
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputNestedListErr<'s,C: Parser<'s>, S: Parser<'s>, I: Parser<'s>> {
    ItemBeforeOpen(I::Output),
    CloseWithoutOutputingOpen(C::Output),
    ExpectedItemOrClose(I::Error, C::Error),
    // ExpectedSeparatorOrClose(S::Error<'s>, C::Error<'s>),
    ExpectedSeparator(S::Error),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShortOutputNestedListErr {
    ItemBeforeOpen,
    CloseWithoutOutputingOpen,
    ExpectedItemOrClose,
    // ExpectedSeparatorOrClose,
    ExpectedSeparator,
}
impl<'s,C: Parser<'s>, S: Parser<'s>, I: Parser<'s>> From<OutputNestedListErr<'s, C, S, I>>
    for ShortOutputNestedListErr
{
    fn from(value: OutputNestedListErr<'s,C, S, I>) -> Self {
        match value {
            OutputNestedListErr::ItemBeforeOpen(_) => Self::ItemBeforeOpen,
            OutputNestedListErr::CloseWithoutOutputingOpen(_) => Self::CloseWithoutOutputingOpen,
            OutputNestedListErr::ExpectedItemOrClose(_, _) => Self::ExpectedItemOrClose,
            // OutputNestedListErr::ExpectedSeparatorOrClose(_, _) => Self::ExpectedSeparatorOrClose,
            OutputNestedListErr::ExpectedSeparator(_) => Self::ExpectedSeparator,
        }
    }
}
pub struct OutputNestedList<T, I, O, C, S> {
    open: O,
    close: C,
    separator: S,
    item: I,
    phantom_collection: PhantomData<T>,
}
impl<T, I, O, C, S> OutputNestedList<T, I, O, C, S> {
    pub fn new(open: O, close: C, separator: S, item: I) -> Self {
        Self {
            open,
            close,
            separator,
            item,
            phantom_collection: PhantomData,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtLeastOneParser<P, C> {
    parser: P,
    phantom_collection: PhantomData<C>,
}
impl<P, C> AtLeastOneParser<P, C> {
    pub fn new(parser: P) -> Self {
        Self {
            parser,
            phantom_collection: PhantomData,
        }
    }
}
impl<'s, P: Parser<'s>, C: FromIterator<P::Output>> Parser<'s> for AtLeastOneParser<P, C>
{
    type Output = (C, Option<P::Error>);
    type Error = P::Error;
    fn parse_next(
        &mut self,
        source: &'s str,
    ) -> Result<ParserOutput<'s, Self::Output>, Self::Error> {
        let mut chars_consumed = 0;
        let mut remaining = source;
        let mut error = None;
        let mut items = std::iter::from_fn(|| match self.parser.parse_next(remaining) {
            Ok(output) => {
                chars_consumed += output.chars_consumed;
                remaining = output.remaining;
                Some(output.output)
            }
            Err(err) => {
                error = Some(err);
                None
            }
        });
        let Some(first) = items.next() else{
            return Err(unsafe { error.unwrap_unchecked() });
        };
        let collection = (std::iter::once(first).chain(items)).collect::<C>();
        Ok(ParserOutput {
            output: (collection, error),
            remaining,
            chars_consumed,
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtLeastOneWithParser<P, C> {
    parser: P,
    collection: C,
}
impl<P, C> AtLeastOneWithParser<P, C> {
    pub fn new(parser: P,collection:C) -> Self {
        Self {
            parser,
            collection,
        }
    }
}
impl<'s, P: Parser<'s>, C: Extend<P::Output>+Clone> Parser<'s> for AtLeastOneWithParser<P, C>
{
    type Output = (C,Option<P::Error>);
    type Error = P::Error;
    fn parse_next(
        &mut self,
        source: &'s str,
    ) -> Result<ParserOutput<'s, Self::Output>, Self::Error> {
        let mut chars_consumed = 0;
        let mut remaining = source;
        let mut error = None;
        let mut items = std::iter::from_fn(|| match self.parser.parse_next(remaining) {
            Ok(output) => {
                chars_consumed += output.chars_consumed;
                remaining = output.remaining;
                Some(output.output)
            }
            Err(err) => {
                error = Some(err);
                None
            }
        });
        let Some(first) = items.next() else{
            return Err(unsafe { error.unwrap_unchecked() });
        };
        let mut collection = self.collection.clone();
        collection.extend(std::iter::once(first).chain(items));
        Ok(ParserOutput {
            output: (collection,error),
            remaining,
            chars_consumed,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ExtendWithAtLeastOneParser<'a, P, C> {
    parser: P,
    collection: &'a mut C,
}
impl<'a,P, C> ExtendWithAtLeastOneParser<'a,P, C> {
    pub fn new(parser: P,collection: &'a mut C) -> Self {
        Self {
            parser,
            collection,
        }
    }
}
impl<'s, P: Parser<'s>, C: Extend<P::Output>> Parser<'s> for ExtendWithAtLeastOneParser<'s,P, C>
{
    type Output = Option<P::Error>;
    type Error = P::Error;
    fn parse_next(
        &mut self,
        source: &'s str,
    ) -> Result<ParserOutput<'s, Self::Output>, Self::Error> {
        let mut chars_consumed = 0;
        let mut remaining = source;
        let mut error = None;
        let mut items = std::iter::from_fn(|| match self.parser.parse_next(remaining) {
            Ok(output) => {
                chars_consumed += output.chars_consumed;
                remaining = output.remaining;
                Some(output.output)
            }
            Err(err) => {
                error = Some(err);
                None
            }
        });
        let Some(first) = items.next() else{
            return Err(unsafe { error.unwrap_unchecked() });
        };
        self.collection.extend(std::iter::once(first).chain(items));
        Ok(ParserOutput {
            output: error,
            remaining,
            chars_consumed,
        })
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManyParser<P, C> {
    parser: P,
    phantom_collection: PhantomData<C>,
}
impl<P, C> ManyParser<P, C> {
    pub fn new(parser: P) -> Self {
        Self {
            parser,
            phantom_collection: PhantomData,
        }
    }
}
impl<'s,P: Parser<'s>, C: FromIterator<P::Output>> Parser<'s> for ManyParser<P, C>
// where
    // for<'s> Result<ParserOutput<'s, P::Output<'s>>, ParserError<P::Error<'s>>>: Debug,
{
    type Output = (C, Option<P::Error>);
    type Error = Infallible;
    fn parse_next(
        &mut self,
        source: &'s str,
    ) -> Result<ParserOutput<'s, Self::Output>, Self::Error> {
        let mut chars_consumed = 0;
        let mut remaining = source;
        let mut error = None;
        let collection = std::iter::from_fn(|| match self.parser.parse_next(remaining) {
            Ok(output) => {
                chars_consumed += output.chars_consumed;
                remaining = output.remaining;
                Some(output.output)
            }
            Err(err) => {
                error = Some(err);
                None
            }
        })
        .collect::<C>();
        Ok(ParserOutput {
            output: (collection, error),
            remaining,
            chars_consumed,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManyWithParser<P, C> {
    parser: P,
    collection: C,
}
impl<P, C> ManyWithParser<P, C> {
    pub fn new(parser: P,collection:C) -> Self {
        Self {
            parser,
            collection,
        }
    }
}
impl<'s,P: Parser<'s>, C: Extend<P::Output>+Clone> Parser<'s> for ManyWithParser<P, C>
{
    type Output = (C, Option<P::Error>);
    type Error = Infallible;
    fn parse_next(
        &mut self,
        source: &'s str,
    ) -> Result<ParserOutput<'s, Self::Output>, Self::Error> {
        let mut chars_consumed = 0;
        let mut remaining = source;
        let mut error = None;
        let mut collection = self.collection.clone();
        collection.extend(std::iter::from_fn(|| match self.parser.parse_next(remaining) {
            Ok(output) => {
                chars_consumed += output.chars_consumed;
                remaining = output.remaining;
                Some(output.output)
            }
            Err(err) => {
                error = Some(err);
                None
            }
        }));
        Ok(ParserOutput {
            output: (collection, error),
            remaining,
            chars_consumed,
        })
    }
}


#[derive(Debug, PartialEq, Eq)]
pub struct ExtendWithManyParser<'a,P, C> {
    parser: P,
    collection:&'a mut C,
}
impl<'a, P, C> ExtendWithManyParser<'a,P, C> {
    pub fn new(parser: P,collection: &'a mut C) -> Self {
        Self {
            parser,
            collection,
        }
    }
}
impl<'s,P: Parser<'s>, C: Extend<P::Output>> Parser<'s> for ExtendWithManyParser<'s,P, C>
{
    type Output = Option<P::Error>;
    type Error = Infallible;
    fn parse_next(
        &mut self,
        source: &'s str,
    ) -> Result<ParserOutput<'s, Self::Output>, Self::Error> {
        let mut chars_consumed = 0;
        let mut remaining = source;
        let mut error = None;
        let items = std::iter::from_fn(|| match self.parser.parse_next(remaining) {
            Ok(output) => {
                chars_consumed += output.chars_consumed;
                remaining = output.remaining;
                Some(output.output)
            }
            Err(err) => {
                error = Some(err);
                None
            }
        });
        self.collection.extend(items);
        
        Ok(ParserOutput {
            output: error,
            remaining,
            chars_consumed,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AndParser<A, B> {
    a: A,
    b: B,
}
impl<A, B> AndParser<A, B> {
    pub fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}
impl<'s,A: Parser<'s>, B: Parser<'s>> Parser<'s> for AndParser<A, B> {
    type Output = (A::Output, B::Output);
    type Error = Or<A::Error, B::Error>;
    fn parse_next(
        &mut self,
        source: &'s str,
    ) -> Result<ParserOutput<'s, Self::Output>, Self::Error> {
        match self.a.parse_next(source) {
            Ok(a_output) => match self.b.parse_next(a_output.remaining) {
                Ok(b_output) => Ok(ParserOutput {
                    output: (a_output.output, b_output.output),
                    remaining: b_output.remaining,
                    chars_consumed: a_output.chars_consumed + b_output.chars_consumed,
                }),
                Err(b_err) => Err(Or::Right(b_err)),
            },
            Err(a_err) => Err(Or::Left(a_err)),
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapParser<P, F> {
    parser: P,
    map: F,
}
impl<P, F> MapParser<P, F> {
    pub fn new(parser: P, map: F) -> Self {
        Self { parser, map }
    }
}
impl <'s,P:Parser<'s>,F:FnMut(P::Output)->U,U> Parser<'s> for MapParser<P,F> {
    type Output=U;
    type Error=P::Error;
    fn parse_next(
        &mut self,
        source: &'s str,
    ) -> Result<ParserOutput<'s, Self::Output>, Self::Error> {
        self.parser.parse_next(source).map(|out|out.map(&mut self.map))
    }
} 

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapErrParser<P, F> {
    parser: P,
    map_err: F,
}
impl<P, F> MapErrParser<P, F> {
    pub fn new(parser: P, map_err: F) -> Self {
        Self { parser, map_err }
    }
}
impl <'s,P:Parser<'s>,F:FnMut(P::Error)->U,U> Parser<'s> for MapErrParser<P,F> {
    type Output=P::Output;
    type Error=U;
    fn parse_next(
        &mut self,
        source: &'s str,
    ) -> Result<ParserOutput<'s, Self::Output>, Self::Error> {
        self.parser.parse_next(source).map_err(&mut self.map_err)
    }
} 

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapBothParser<P, F, G> {
    parser: P,
    map: F,
    map_err: G,
}
impl<P, F,G> MapBothParser<P, F,G> {
    pub fn new(parser: P, map: F,map_err:G) -> Self {
        Self { parser, map,map_err }
    }
}
impl <'s,P:Parser<'s>,F:FnMut(P::Output)->U,G:FnMut(P::Error)->V,U,V> Parser<'s> for MapBothParser<P,F,G> {
    type Output=U;
    type Error=V;
    fn parse_next(
        &mut self,
        source: &'s str,
    ) -> Result<ParserOutput<'s, Self::Output>, Self::Error> {
        self.parser.parse_next(source).map(|out|out.map(&mut self.map)).map_err(&mut self.map_err)
    }
} 

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OneOfParser<T> {
    parsers:T
}
impl<T> OneOfParser<T> {
    pub fn new(parsers:T) -> Self {
        Self { parsers }
    }
}
impl <'s,T:TupleOfParsers<'s>> Parser<'s> for OneOfParser<T> {
    type Output=T::OneOfOutput;

    type Error=T::OneOfError;

    fn parse_next(
        &mut self,
        source: &'s str,
    ) -> Result<ParserOutput<'s, Self::Output>, Self::Error> {
        T::parse_one_of(&mut self.parsers, source)
    }
}

pub trait TupleOfParsers<'s>{
    type OneOfOutput;
    type OneOfError;
    fn parse_one_of(
        &mut self,
        source: &'s str,
    ) -> Result<ParserOutput<'s, Self::OneOfOutput>, Self::OneOfError>;

    fn one_of(self) -> OneOfParser<Self>
        where Self:Sized
    {
        OneOfParser { parsers: self }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrParser<L, R> {
    left: L,
    right: R,
}
impl<L, R> OrParser<L, R> {
    pub fn new(left: L, right: R) -> Self {
        Self { left, right }
    }
}
impl<'s,L: Parser<'s>, R: Parser<'s>> Parser<'s> for OrParser<L, R> {
    type Output = Or<L::Output, R::Output>;
    type Error = (L::Error, R::Error);
    fn parse_next(
        &mut self,
        source: &'s str,
    ) -> Result<ParserOutput<'s, Self::Output>, Self::Error> {
        match self.left.parse_next(source) {
            Ok(l) => Ok(ParserOutput {
                output: Or::Left(l.output),
                remaining: l.remaining,
                chars_consumed: l.chars_consumed,
            }),
            Err(l_err) => match self.right.parse_next(source) {
                Ok(r) => Ok(ParserOutput {
                    output: Or::Right(r.output),
                    remaining: r.remaining,
                    chars_consumed: r.chars_consumed,
                }),
                Err(r_err) => Err((l_err, r_err)),
            },
        }
    }
}

pub struct CharBoundaries<'s> {
    source: &'s str,
    char_boundary: Option<usize>,
}
impl<'s> From<&'s str> for CharBoundaries<'s> {
    fn from(source: &'s str) -> Self {
        Self {
            source,
            char_boundary: Some(0),
        }
    }
}
impl <'s> Iterator for CharBoundaries<'s> {
    type Item = (usize, &'s str);

    fn next(&mut self) -> Option<Self::Item> {
        let char_boundary = self.char_boundary?;
        self.char_boundary = (char_boundary + 1..=self.source.len())
            .find(|&index| self.source.is_char_boundary(index));
        Some((char_boundary, &self.source[..char_boundary]))
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FromStrParser<T>(PhantomData<T>);
impl<P> FromStrParser<P> {
    pub const PARSER: Self = Self(PhantomData);
}

impl<'s, P: FromStr> Parser<'s> for FromStrParser<P> {
    type Output = P;
    type Error = P::Err;
    fn parse_next(
        &mut self,
        source: &'s str,
    ) -> Result<ParserOutput<'s, Self::Output>, Self::Error> {
        let mut boundaries = CharBoundaries::from(source).enumerate();
        while let Some((char_index, (byte_index, current_str))) = boundaries.next() {
            if let Ok(output) = current_str.parse::<P>().map(|output| ParserOutput {
                chars_consumed: char_index,
                output,
                remaining: &source[byte_index..],
            }) {
                let mut to_return = output;
                while let Some((char_index, (byte_index, current_str))) = boundaries.next() {
                    if let Ok(output) = current_str.parse::<P>().map(|output| ParserOutput {
                        chars_consumed: char_index,
                        output,
                        remaining: &source[byte_index..],
                    }) {
                        to_return = output
                    } else {
                        break;
                    }
                }
                return Ok(to_return);
            }
        }
        source
            .parse::<P>()
            .map(|output| ParserOutput {
                chars_consumed: source.len(),
                output,
                remaining: "",
            })
    }
}
impl<'s,P: Parser<'s>> Parser<'s> for &mut P {
    type Error = P::Error;
    type Output = P::Output;
    fn parse_next(
        &mut self,
        source: &'s str,
    ) -> Result<ParserOutput<'s, Self::Output>, Self::Error> {
        P::parse_next(self, source)
    }
}
// impl <T:TupleOfParsers> Parser for T {
//     type Output<'o>= T::AllOutput<'o>;

//     type Error<'e>= T::AllError<'e>;

//     fn parse_next<'s>(
//         &mut self,
//         source: &'s str,
//     ) -> Result<ParserOutput<'s, Self::Output<'s>>, ParserError<Self::Error<'s>>> {
//         self.parse_all(source)
//     }
// }

impl<'s,
        O: Parser<'s>,
        C: Parser<'s>,
        S: Parser<'s>,
        I: Parser<'s>,
        T: Default + Extend<I::Output> + Extend<T>,
    > Parser<'s> for OutputNestedList<T, I, O, C, S>
// where
//     O::Output: Debug,
//     I::Output: Debug,
{
    type Output = T;
    type Error = WithLocation<OutputNestedListErr<'s, C, S, I>>;
    fn parse_next(
        &mut self,
        string: &'s str,
    ) -> Result<ParserOutput<'s, Self::Output>, Self::Error> {
        let mut remaining_string = string;
        let mut total_chars_consumed = 0;
        let mut collections = Vec::<T>::default();

        loop {
            // Parse a bunch of opens
            while let Ok(ParserOutput {
                remaining,
                chars_consumed,
                ..
            }) = self.open.parse_next(remaining_string)
            {
                remaining_string = remaining;
                total_chars_consumed += chars_consumed;
                collections.push(T::default())
            }
            // Parse item
            let mut item_or_close = OrParser::new(&mut self.item, &mut self.close);
            let ParserOutput {
                output: item_or_close,
                remaining,
                chars_consumed,
            } = item_or_close.parse_next(remaining_string)
            //     .map_err(|e| {
            //     e.map(|(i, c)| OutputNestedListErr::ExpectedItemOrClose(i, c))
            //         .with_offset(total_chars_consumed)
            // })?;
                .map_err(|(i,c)|WithLocation::new(total_chars_consumed,OutputNestedListErr::ExpectedItemOrClose(i, c)))?;
            remaining_string = remaining;
            total_chars_consumed += chars_consumed;
            if let Or::Left(item) = item_or_close {
                if let Some(last_mut) = collections.last_mut() {
                    last_mut.extend([item]);
                } else {
                    break Err(WithLocation::new(
                        total_chars_consumed,
                        OutputNestedListErr::ItemBeforeOpen(item),
                    ));
                }
            } else if let Or::Right(close_output) = item_or_close {
                let Some(closed) = collections.pop() else {
                    return Err(WithLocation::new(total_chars_consumed,OutputNestedListErr::CloseWithoutOutputingOpen(close_output)))
                };
                if let Some(last_mut) = collections.last_mut() {
                    last_mut.extend([closed]);
                } else {
                    return Ok(ParserOutput {
                        output: closed,
                        remaining: remaining_string,
                        chars_consumed: total_chars_consumed,
                    });
                }
            }
            while let Ok(ParserOutput {
                output: close_output,
                remaining,
                chars_consumed,
            }) = self.close.parse_next(remaining_string)
            {
                remaining_string = remaining;
                total_chars_consumed += chars_consumed;
                let Some(closed) = collections.pop() else {
                    return Err(WithLocation::new(total_chars_consumed,OutputNestedListErr::CloseWithoutOutputingOpen(close_output)))
                };
                if let Some(last_mut) = collections.last_mut() {
                    last_mut.extend([closed]);
                } else {
                    return Ok(ParserOutput {
                        output: closed,
                        remaining: remaining_string,
                        chars_consumed: total_chars_consumed,
                    });
                }
            }
            let ParserOutput {
                remaining,
                chars_consumed,
                ..
            } = self.separator.parse_next(remaining_string)
            // .map_err(|e| {
            //     e.map(|s| OutputNestedListErr::ExpectedSeparator(s))
            //         .with_offset(total_chars_consumed)
            // })?;
            .map_err(|e|WithLocation::new(total_chars_consumed, OutputNestedListErr::ExpectedSeparator(e)))?;
            remaining_string = remaining;
            total_chars_consumed += chars_consumed;
        }
    }
}

#[derive(Debug, Clone)]
pub struct Delimeted<C, I, D> {
    item: I,
    delimeter: D,
    phantom_collection: PhantomData<C>,
}
impl<I, D, C> Delimeted<C, I, D> {
    pub fn new(item: I, delimeter: D) -> Self {
        Self {
            delimeter,
            item,
            phantom_collection: PhantomData,
        }
    }
}

#[derive(Debug)]
pub enum DelimetedError<'e, I: Parser<'e>, D: Parser<'e>> {
    ExpectedItem(I::Error),
    ExpectedDelimeter(D::Error),
}
impl<'s,I: Parser<'s> + Debug, D: Parser<'s> + Debug, C: Default + for<'a> Extend<I::Output>> Parser<'s>
    for Delimeted<C, I, D>
// where
//     I::Error: Debug,
//     D::Error: Debug,
//     I::Output: Debug,
//     D::Output: Debug,
//     C: Debug,
{
    type Output = C;

    type Error = WithLocation<DelimetedError<'s, I, D>>;

    fn parse_next(
        &mut self,
        source: &'s str,
    ) -> Result<ParserOutput<'s, Self::Output>,Self::Error> {
        let mut collection = C::default();
        let first_item = self
            .item
            .parse_next(source)
            .map_err(|e|WithLocation::new(0,DelimetedError::ExpectedItem(e)))?;
        collection.extend([first_item.output]);
        let mut remaining = first_item.remaining;
        let mut chars_consumed = first_item.chars_consumed;

        loop {
            // println!("Test {:?}",collection);
            let Ok(next_delim) = self.delimeter.parse_next(remaining).map_err(|e|WithLocation::new(chars_consumed, DelimetedError::<I,D>::ExpectedDelimeter(e))) else {
                break;
            };
            chars_consumed += next_delim.chars_consumed;
            remaining = next_delim.remaining;

            let next_item = self.item.parse_next(remaining)
            // .map_err(|e| {
            //     e.map(DelimetedError::<I, D>::ExpectedItem)
            //         .with_offset(chars_consumed)
            // }))?;
            .map_err(|e|WithLocation::new(chars_consumed, DelimetedError::<I, D>::ExpectedItem(e)))?;
            collection.extend([next_item.output]);
            chars_consumed += next_item.chars_consumed;
            remaining = next_item.remaining;
        }
        Ok(ParserOutput {
            output: collection,
            remaining,
            chars_consumed,
        })
    }
}

#[derive(Debug, Clone)]
pub struct DelimetedArray<const N: usize, I, D> {
    item: I,
    delimeter: D,
}
impl<const N: usize, I, D> DelimetedArray<N, I, D> {
    pub fn new(item: I, delimeter: D) -> Self {
        Self { delimeter, item }
    }
}

#[derive(Debug)]
pub enum DelimetedArrayError<'s, const N: usize, I: Parser<'s>, D: Parser<'s>> {
    ExpectedItem { index: usize, error: I::Error },
    ExpectedDelimeter(D::Error),
}
impl<'s,const N: usize, I: Parser<'s> + Debug, D: Parser<'s> + Debug> Parser<'s> for DelimetedArray<N, I, D>
// where
//     I::Error: Debug,
//     D::Error: Debug,
//     I::Output: Debug,
//     D::Output: Debug,
{
    type Output = [I::Output; N];

    type Error = WithLocation<DelimetedArrayError<'s, N, I, D>>;

    fn parse_next(
        &mut self,
        source: &'s str,
    ) -> Result<ParserOutput<'s, Self::Output>,Self::Error> {
        let mut remaining = source;
        let mut chars_consumed = 0;
        let mut items = [(); N].map(|_| MaybeUninit::uninit());
        if N != 0 {
            let first_item = self.item.parse_next(source).map_err(|error| {
                WithLocation::new(0,DelimetedArrayError::ExpectedItem { error, index: 0 })
            })?;
            let mut item_iter = items.iter_mut().enumerate();
            if let Some((_, first_slot)) = item_iter.next() {
                first_slot.write(first_item.output);
            }

            remaining = first_item.remaining;
            chars_consumed = first_item.chars_consumed;
            for (index, item) in item_iter {
                let Ok(next_delim) = self.delimeter.parse_next(remaining).map_err(|e|WithLocation::new(chars_consumed,DelimetedArrayError::<N,I,D>::ExpectedDelimeter(e))) else {
                    break;
                };

                chars_consumed += next_delim.chars_consumed;
                remaining = next_delim.remaining;

                let next_item = self.item.parse_next(remaining)
                // .map_err(|e| {
                //     e.map(|error| DelimetedArrayError::ExpectedItem { error, index })
                //         .with_offset(chars_consumed)
                // }))?;
                .map_err(|error|WithLocation::new(chars_consumed,DelimetedArrayError::ExpectedItem { error, index }))?;
                item.write(next_item.output);
                chars_consumed += next_item.chars_consumed;
                remaining = next_item.remaining;
            }
        }
        Ok(ParserOutput {
            output: items.map(|item| unsafe { item.assume_init() }),
            remaining,
            chars_consumed,
        })
    }
}




macro_rules! nest_or {
    ($first:ty,$($rest:ty),+)=>{
        Or<$first,nest_or!($($rest),+)>
    };
    ($first:ty)=>{
        $first
    };
    ()=>{()}
}

macro_rules! parse_one_of {
    ([$($errs:ident)*] $remaining:ident,$chars_consumed:ident; $first:ident $(,$rest:ident)+)=>{
        match $first.parse_next($remaining) {
            Ok(out)=> {
                $remaining = out.remaining;
                $chars_consumed = out.chars_consumed;
                Or::Left(out.output)
            },
            Err($first)=>Or::Right(parse_one_of!([$($errs)* $first] $remaining,$chars_consumed;$($rest),+))
        }
    };

    ([$($errs:ident)*] $remaining:ident,$chars_consumed:ident; $last:ident)=>{
        match $last.parse_next($remaining) {
            Ok(out)=> {
                $remaining = out.remaining;
                $chars_consumed = out.chars_consumed;
                out.output
            },
            Err($last)=>{
                return Err(($($errs,)*$last,))
            }
        }
    };
}

macro_rules! parse_all {
    ([$($outs:ident)*] $remaining:ident,$chars_consumed:ident; $first:ident $(,$rest:ident)+)=>{
        match $first.parse_next($remaining) {
            Ok($first)=> {
                $remaining = $first.remaining;
                $chars_consumed += $first.chars_consumed;
                let $first = $first.output;
                Or::Right(parse_all!([$($outs)* $first] $remaining,$chars_consumed;$($rest),+))
            },
            Err(err)=>Or::Left(err)
        }
    };

    ([$($outs:ident)*] $remaining:ident,$chars_consumed:ident; $last:ident)=>{
        match $last.parse_next($remaining) {
            Ok(out)=> {
                $remaining = out.remaining;
                $chars_consumed += out.chars_consumed;
                return Ok(ParserOutput{
                    output:($($outs,)* out.output,),
                    remaining:$remaining,
                    chars_consumed:$chars_consumed,
                })
            },
            Err(err)=>err
        }
    };

    ([$($outs:ident)*] $remaining:ident,$chars_consumed:ident; )=>{
        match $last.parse_next($remaining) {
            Ok(out)=> {
                $remaining = out.remaining;
                $chars_consumed += out.chars_consumed;
                return Ok(ParserOutput{
                    output:($($outs,)* out.output,),
                    remaining:$remaining,
                    chars_consumed:$chars_consumed,
                })
            },
            Err(err)=>err
        }
    };
}

macro_rules! tuple_parsers {
    ($($parser_name:ident $parser_type:ident)*) => {
        impl <'s,$($parser_type:Parser<'s>),*> TupleOfParsers<'s> for ($($parser_type,)*) {
            type OneOfOutput = nest_or!($(<$parser_type as Parser<'s>>::Output),*);
            type OneOfError = ($(<$parser_type as Parser<'s>>::Error,)*);

            fn parse_one_of(
                &mut self,
                source: &'s str,
            ) -> Result<ParserOutput<'s, Self::OneOfOutput>, Self::OneOfError>{
                let ($($parser_name,)*) = self;
                let mut remaining=source;
                let chars_consumed;
                let output = parse_one_of!([] remaining,chars_consumed; $($parser_name),*);
                
                Ok(ParserOutput {
                    output,
                    remaining,
                    chars_consumed,
                })
            }
        }
        impl <'s,$($parser_type:Parser<'s>),*> Parser<'s> for ($($parser_type,)*) {
            type Output = ($(<$parser_type as Parser<'s>>::Output,)*);
            type Error = WithLocation<nest_or!($(<$parser_type as Parser<'s>>::Error),*)>;

            fn parse_next(
                &mut self,
                source: &'s str,
            ) -> Result<ParserOutput<'s, Self::Output>, Self::Error>{
                let ($($parser_name,)*) = self;
                let mut remaining=source;
                let mut chars_consumed = 0;
                let err = parse_all!([] remaining,chars_consumed; $($parser_name),*);
                
                Err(WithLocation::new(chars_consumed,err))
            }
        }
    };
    // ($first_parser_name:ident $first_parser_type:ident) => {
    //     impl <$first_parser_type:Parser> TupleOfParsers for ($first_parser_type,) {
    //         type OneOfOutput<'o> = nest_or!(first_parser_type)
    //     }
    // };
    () => {
        
    };
}

impl <'s,const N:usize,P:Parser<'s>> TupleOfParsers<'s> for [P;N] {
    type OneOfOutput = P::Output;

    type OneOfError = [P::Error;N];

    fn parse_one_of(
        &mut self,
        source: &'s str,
    ) -> Result<ParserOutput<'s, Self::OneOfOutput>, Self::OneOfError> {
        let mut errors = [();N].map(|_|MaybeUninit::uninit());
        let mut initialized = 0;

        for (parser,error_slot) in self.iter_mut().zip(errors.iter_mut()){
            match parser.parse_next(source) {
                Ok(value) => {
                    for i in &mut errors[..initialized] {
                        unsafe{i.assume_init_drop()};
                    }
                    return Ok(value)
                },
                Err(error) => {
                    initialized+=1;
                    error_slot.write(error);
                }
            }
        }

        Err(errors.map(|e|unsafe{e.assume_init()}))
    }
}

impl <'s,const N:usize,P:Parser<'s>> Parser<'s> for [P;N] {
    type Output=[P::Output;N];

    type Error = WithLocation<P::Error>;

    fn parse_next(
        &mut self,
        source: &'s str,
    ) -> Result<ParserOutput<'s, Self::Output>, Self::Error> {
        let mut outputs = [();N].map(|_|MaybeUninit::uninit());
        let mut initialized = 0;

        let mut remaining = source;
        let mut chars_consumed = 0;

        for (parser,output_slot) in self.iter_mut().zip(outputs.iter_mut()){
            match parser.parse_next(source) {
                Ok(value) => {
                    chars_consumed+=value.chars_consumed;
                    remaining = value.remaining;
                    output_slot.write(value.output);
                    initialized+=1;
                },
                Err(error) => {
                    for i in &mut outputs[..initialized] {
                        unsafe{i.assume_init_drop()};
                    }
                    return Err(WithLocation::new(chars_consumed,error))
                }
            }
        }

        Ok(ParserOutput { output: outputs.map(|e|unsafe{e.assume_init()}),chars_consumed,remaining })
    }
}

pub struct WhileMatchParser<F>(F);
pub fn while_match<F>(f:F)->WhileMatchParser<F>{
    WhileMatchParser(f)
}
impl <'s,F:FnMut(char)->bool> Parser<'s> for WhileMatchParser<F>{
    type Output = &'s str;
    type Error = Infallible;
    fn parse_next(
            &mut self,
            source: &'s str,
        ) -> Result<ParserOutput<'s, Self::Output>, Self::Error> {
        let mut index = source.char_indices().enumerate().find_map(|(char_index,(byte_index,c))|(!(self.0)(c)).then_some((char_index,byte_index)));
        match index {
            Some((char_index,byte_index))=>{
                Ok(ParserOutput { output: &source[..byte_index], remaining: &source[byte_index..], chars_consumed: char_index })
            }
            None=>Ok(ParserOutput { output: &source, remaining: &source[source.len().saturating_sub(1)..], chars_consumed: source.chars().count() })
        }
    }
}

pub struct MaybeParser<P>(P);
impl <'s,P:Parser<'s>> Parser<'s> for MaybeParser<P>{
    type Output = Option<P::Output>;
    type Error = Infallible;
    fn parse_next(
            &mut self,
            source: &'s str,
        ) -> Result<ParserOutput<'s, Self::Output>, Self::Error> {
        if let Ok(ParserOutput { output, remaining, chars_consumed })=self.0.parse_next(source){
            Ok(ParserOutput{output:Some(output),remaining,chars_consumed})
        }else {
            Ok(ParserOutput{output:None,remaining:source,chars_consumed:0})
        }
    }
}
pub fn maybe<P>(p:P)->MaybeParser<P>{
    MaybeParser(p)
}

#[derive(Debug,Clone, Copy,PartialEq, Eq)]
pub struct DigitParser;
impl <'s> Parser<'s> for DigitParser {
    type Output = u8;
    type Error = Option<char>;
    fn parse_next(
            &mut self,
            source: &'s str,
        ) -> Result<ParserOutput<'s, Self::Output>, Self::Error> {
        let mut chars = source.chars();
        let first_char = chars.next();
        if let Some(c@'0'..='9') = first_char {
            Ok(ParserOutput {
                output: c as u8 - b'0',
                remaining: chars.as_str(),
                chars_consumed: 1,
            })
        } else {
            Err(first_char)
        }
    }
}
#[derive(Debug,Clone, Copy,PartialEq, Eq)]
pub struct OctalDigitParser;
impl <'s> Parser<'s> for OctalDigitParser {
    type Output = u8;
    type Error = Option<char>;
    fn parse_next(
            &mut self,
            source: &'s str,
        ) -> Result<ParserOutput<'s, Self::Output>, Self::Error> {
        let mut chars = source.chars();
        let first_char = chars.next();
        if let Some(c@'0'..='7') = first_char {
            Ok(ParserOutput {
                output: c as u8 - b'0',
                remaining: chars.as_str(),
                chars_consumed: 1,
            })
        } else {
            Err(first_char)
        }
    }
}

#[derive(Debug,Clone, Copy,PartialEq, Eq)]
pub struct HexDigitParser;
impl <'s> Parser<'s> for HexDigitParser {
    type Output = u8;
    type Error = Option<char>;
    fn parse_next(
            &mut self,
            source: &'s str,
        ) -> Result<ParserOutput<'s, Self::Output>, Self::Error> {
        let mut chars = source.chars();
        let first_char = chars.next();
        if let Some(c@'0'..='9') = first_char {
            Ok(ParserOutput {
                output: c as u8 - b'0',
                remaining: chars.as_str(),
                chars_consumed: 1,
            })
        } else if let Some(c@'a'..='f') = first_char {
            Ok(ParserOutput {
                output: c as u8 - b'a'+10,
                remaining: chars.as_str(),
                chars_consumed: 1,
            })
        } else if let Some(c@'A'..='F') = first_char {
            Ok(ParserOutput {
                output: c as u8 - b'A'+10,
                remaining: chars.as_str(),
                chars_consumed: 1,
            })
        } else {
            Err(first_char)
        }
    }
}


tuple_parsers!(a A);
tuple_parsers!(a A b B);
tuple_parsers!(a A b B c C);
tuple_parsers!(a A b B c C d D);
tuple_parsers!(a A b B c C d D e E);
tuple_parsers!(a A b B c C d D e E f F);
tuple_parsers!(a A b B c C d D e E f F g G);
tuple_parsers!(a A b B c C d D e E f F g G h H);
tuple_parsers!(a A b B c C d D e E f F g G h H i I);
tuple_parsers!(a A b B c C d D e E f F g G h H i I j J);
tuple_parsers!(a A b B c C d D e E f F g G h H i I j J k K);
tuple_parsers!(a A b B c C d D e E f F g G h H i I j J k K l L);
tuple_parsers!(a A b B c C d D e E f F g G h H i I j J k K l L m M);
tuple_parsers!(a A b B c C d D e E f F g G h H i I j J k K l L m M n N);
tuple_parsers!(a A b B c C d D e E f F g G h H i I j J k K l L m M n N o P);
tuple_parsers!(a A b B c C d D e E f F g G h H i I j J k K l L m M n N o P q Q);
tuple_parsers!(a A b B c C d D e E f F g G h H i I j J k K l L m M n N o P q Q r R);
tuple_parsers!(a A b B c C d D e E f F g G h H i I j J k K l L m M n N o P q Q r R s S);
tuple_parsers!(a A b B c C d D e E f F g G h H i I j J k K l L m M n N o P q Q r R s S t T);
tuple_parsers!(a A b B c C d D e E f F g G h H i I j J k K l L m M n N o P q Q r R s S t T u U);
tuple_parsers!(a A b B c C d D e E f F g G h H i I j J k K l L m M n N o P q Q r R s S t T u U v V);
tuple_parsers!(a A b B c C d D e E f F g G h H i I j J k K l L m M n N o P q Q r R s S t T u U v V w W);
tuple_parsers!(a A b B c C d D e E f F g G h H i I j J k K l L m M n N o P q Q r R s S t T u U v V w W x X);
tuple_parsers!(a A b B c C d D e E f F g G h H i I j J k K l L m M n N o P q Q r R s S t T u U v V w W x X y Y);
tuple_parsers!(a A b B c C d D e E f F g G h H i I j J k K l L m M n N o P q Q r R s S t T u U v V w W x X y Y z Z);