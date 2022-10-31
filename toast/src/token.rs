use std::fmt::{Debug, Formatter};
use clap::Id;

use crate::try_parse_from_iter::{Peek,TryParseFromPeek};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BracketType {
    Parenthesis,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

impl Debug for Operator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Add => "+",
            Self::Sub => "-",
            Self::Mul => "*",
            Self::Div => "/",
            Self::Mod => "%",
        };
        write!(f, "[Operator: {}]", s)
    }
}

pub enum OperatorError {
    Unknown(char),
    NoChar,
}
impl TryFrom<char> for Operator {
    type Error = OperatorError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '-' => Ok(Self::Sub),
            '*' => Ok(Self::Mul),
            '+' => Ok(Self::Add),
            '/' => Ok(Self::Div),
            '%' => Ok(Self::Mod),
            _ => Err(OperatorError::Unknown(value)),
        }
    }
}
impl TryParseFromPeek<char> for Operator {
    type Err = OperatorError;
    type ParseContext = ();
    fn try_parse_from_peek<P: Peek<Item = char>>(
        peek: &mut P,
        _context: Self::ParseContext,
    ) -> Result<Self, Self::Err> {
        let c = peek.peek().cloned().ok_or(OperatorError::NoChar)?;
        let op = Operator::try_from(c);
        if op.is_ok() {
            peek.advance();
        }
        op
    }
}
impl BracketType {
    // #[inline]
    // pub const fn open(&self) -> char {
    //     match self {
    //         Self::Parenthesis => '(',
    //     }
    // }
    // #[inline]
    // pub const fn close(&self) -> char {
    //     match self {
    //         Self::Parenthesis => ')',
    //     }
    // }
}
impl TryFrom<char> for Bracket {
    type Error = char;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '(' => Ok(Self::Open(BracketType::Parenthesis)),
            ')' => Ok(Self::Close(BracketType::Parenthesis)),
            c => Err(c),
        }
    }
}

impl TryParseFromPeek<char> for Bracket {
    type Err = Option<char>;
    type ParseContext = ();
    fn try_parse_from_peek<P: Peek<Item = char>>(
        peek: &mut P,
        _context: Self::ParseContext,
    ) -> Result<Self, Self::Err> {
        let c = *peek.peek().ok_or(None)?;
        Bracket::try_from(c).map_err(Some).map(|c| {
            peek.advance();
            c
        })
    }
}

#[derive(Clone,PartialEq, Eq)]
pub struct IdentifierLike{
    pub name:String
}
impl From<String> for IdentifierLike{
    fn from(name: String) -> Self {
        Self{
            name
        }
    }
}
impl Debug for IdentifierLike {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"[Identifier: {}]",self.name)
    }
}
pub enum IdentifierError {
    InvalidIdentifierCharacter(char),
    Empty,
}
#[inline]
pub fn is_ident_char(c:&char)->bool{
    c.is_numeric() || is_first_ident_char(c)
}
#[inline]
pub fn is_first_ident_char(c:&char)->bool{
    c.is_alphabetic() || *c=='_'
}
impl TryParseFromPeek<char> for IdentifierLike {
    type Err=IdentifierError;

    type ParseContext=();

    fn try_parse_from_peek<P: Peek<Item = char>>(
        peek: &mut P,
        _: Self::ParseContext,
    ) -> Result<Self, Self::Err> {
        match peek.peek() {
            Some(f) => {
                if is_first_ident_char(f) {
                    let mut name = String::new();
                    name.extend(peek.peek_while(|c|{
                        // Some(*c).filter(is_ident_char)
                        is_ident_char(c).then_some(*c)
                    }));
                    Ok(Self { name })
                }else {
                    Err(Self::Err::InvalidIdentifierCharacter(*f))
                }
            },
            None=>Err(Self::Err::Empty)
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SourceLocation {
    pub line: usize,
    pub column: usize,
    length: usize,
}
impl SourceLocation {
    pub fn new(line: usize, column: usize, length: usize) -> Self {
        Self { line, column, length }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Bracket {
    Open(BracketType),
    Close(BracketType),
}
impl Debug for Bracket {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (state_string, bracket_type) = match self {
            Self::Close(ref bracket)=>("Close",bracket),
            Self::Open(ref bracket)=>("Open",bracket),
        };
        write!(f, "[Bracket: {}{:?}]", state_string, bracket_type)
    }
}

type Num =isize;

#[derive(Clone, PartialEq, Eq)]
pub struct NumericLiteral {
    value: isize,
    suffix: Option<String>
}
impl NumericLiteral{
    pub fn new (value: isize,suffix: Option<String>)->Self {
        Self {
            value,
            suffix
        }
    }
}
impl From<Num> for NumericLiteral{
    fn from(value: Num) -> Self {
        Self { value, suffix: None }
    }
}
impl Debug for NumericLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"[Number: {}{}]",self.value, match self.suffix {
            Some(ref s) => s,
            None => ""
        })
    }
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum NumericLiteralError {
    Value(std::num::IntErrorKind),
    // BadSuffix(String),
}
impl TryParseFromPeek<char> for NumericLiteral {
    type Err=NumericLiteralError;

    type ParseContext=u32;

    fn try_parse_from_peek<P: Peek<Item = char>>(
        peek: &mut P,
        context: Self::ParseContext,
    ) -> Result<Self, Self::Err> {
        isize::try_parse_from_peek(peek, context).map_err(NumericLiteralError::Value).and_then(|value|{
            let mut non_whitespace = peek.peek_while(|c|(!c.is_whitespace()).then_some(*c));
            let suffix = non_whitespace.next().map(|first|{
                non_whitespace.fold(first.to_string(),|mut suffix, c|{
                    suffix.push(c);
                    suffix
                })
            });
           Ok(Self {
                value,
                suffix
            })
        })
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum TokenData {
    Bracket(Bracket),
    Number(NumericLiteral),
    Operator(Operator),
    Identifier(IdentifierLike),
}
impl From<Bracket> for TokenData{
    fn from(bracket: Bracket) -> Self {
        Self::Bracket(bracket)
    }
}
impl <N> From<N> for TokenData where NumericLiteral:From<N>{
    fn from(number: N) -> Self {
        Self::Number(number.into())
    }
}
impl From<Operator> for TokenData{
    fn from(operator: Operator) -> Self {
        Self::Operator(operator)
    }
}
impl From<IdentifierLike> for TokenData{
    fn from(identifier: IdentifierLike) -> Self {
        Self::Identifier(identifier)
    }
}
impl Debug for TokenData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bracket(b) => Debug::fmt(b, f),
            Self::Operator(op) => Debug::fmt(op, f),
            Self::Identifier(ident) => Debug::fmt(ident, f),
            Self::Number(n) => write!(f, "[Number: {:?}]", n),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ParseTokenErr {
    UnexpectedCharacter(char),
    Empty,
}

impl TryParseFromPeek<char> for TokenData {
    type Err = ParseTokenErr;
    type ParseContext = ();
    fn try_parse_from_peek<P: Peek<Item = char>>(
        line: &mut P,
        _context: Self::ParseContext,
    ) -> Result<Self, Self::Err> {
        let first_char = line.peek().ok_or(Self::Err::Empty)?;
        if *first_char == '-' {
            NumericLiteral::try_parse_from_peek(line, 10)
                .map(TokenData::Number)
                .or(Ok(TokenData::Operator(Operator::Sub)))
        } else {
            NumericLiteral::try_parse_from_peek(line, 10)
                .map(TokenData::Number)
                .or_else(|e| {
                    dbg!(e);
                    Operator::try_parse_from_peek(line, ())
                        .map(TokenData::Operator)
                        .or_else(|e| match e {
                            OperatorError::NoChar => Err(Self::Err::Empty),
                            _ => Bracket::try_parse_from_peek(line, ())
                                .map(TokenData::Bracket)
                                .or_else(|e| {
                                    if let Some(c) = e {
                                        if let Ok(id)=IdentifierLike::try_parse_from_peek(line, ()){
                                            Ok(TokenData::Identifier(id))
                                        }else{
                                            Err(Self::Err::UnexpectedCharacter(c))
                                        }
                                    } else {
                                        Err(Self::Err::Empty)
                                    }
                                }),
                        })
                })
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Token {
    pub locaiton: SourceLocation,
    pub token_data: TokenData,
}

impl Token {
    pub fn new(location: SourceLocation, data: TokenData) -> Self {
        Self {
            locaiton: location,
            token_data: data,
        }
    }
}
