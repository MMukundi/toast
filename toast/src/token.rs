use crate::try_parse_from_iter::TryParseFromPeek;
use std::fmt::{Debug, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BracketState {
    Open,
    Close,
}

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
    fn try_parse_from_peek<P: crate::try_parse_from_iter::Peek<Item = char>>(
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
    fn try_parse_from_peek<P: crate::try_parse_from_iter::Peek<Item = char>>(
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

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FileLocation {
    pub line: usize,
    pub column: usize,
    length: usize,
}
impl FileLocation {
    pub fn new(line: usize, column: usize, length: usize) -> Self {
        Self { line, column, length }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Bracket {
    Open(BracketType),
    Close(BracketType),
}
impl Bracket {
    #[inline]
    pub const fn new_open(bracket: BracketType) -> Self {
        Self::Open(bracket)
    }
    #[inline]
    pub const fn new_close(bracket: BracketType) -> Self {
        Self::Close(bracket)
    }
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

#[derive(Clone, PartialEq, Eq)]
pub enum TokenData {
    Bracket(Bracket),
    Number(isize),
    Operator(Operator),
}
impl Debug for TokenData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bracket(b) => Debug::fmt(b, f),
            Self::Number(n) => write!(f, "[Number: {:?}]", n),
            Self::Operator(op) => Debug::fmt(op, f),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ParseTokenErr {
    Empty,
    UnexpectedCharacter(char),
}
impl TryParseFromPeek<char> for TokenData {
    type Err = ParseTokenErr;
    type ParseContext = ();
    fn try_parse_from_peek<P: crate::try_parse_from_iter::Peek<Item = char>>(
        line: &mut P,
        _context: Self::ParseContext,
    ) -> Result<Self, Self::Err> {
        let first_char = line.peek().ok_or(Self::Err::Empty)?;
        if *first_char == '-' {
            isize::try_parse_from_peek(line, 10)
                .map(TokenData::Number)
                .or(Ok(TokenData::Operator(Operator::Sub)))
        } else {
            isize::try_parse_from_peek(line, 10)
                .map(TokenData::Number)
                .or_else(|_| {
                    Operator::try_parse_from_peek(line, ())
                        .map(TokenData::Operator)
                        .or_else(|e| match e {
                            OperatorError::NoChar => Err(Self::Err::Empty),
                            _ => Bracket::try_parse_from_peek(line, ())
                                .map(TokenData::Bracket)
                                .map_err(|e| {
                                    if let Some(c) = e {
                                        Self::Err::UnexpectedCharacter(c)
                                    } else {
                                        Self::Err::Empty
                                    }
                                }),
                        })
                })
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Token {
    pub locaiton: FileLocation,
    pub token_data: TokenData,
}

impl Token {
    pub fn new(location: FileLocation, data: TokenData) -> Self {
        Self {
            locaiton: location,
            token_data: data,
        }
    }
}
