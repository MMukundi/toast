use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

use crate::try_parse_from_iter::TryParseFromPeek;

#[derive(Debug,Copy, Clone,Eq, PartialEq)]
pub enum BracketState {
    Open,
    Close
}

#[derive(Debug,Copy, Clone,Eq, PartialEq)]
pub enum Bracket {
    Parenthesis,
}

#[derive(Copy, Clone,Eq, PartialEq)]
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
        write!(f,"[Operator: {}]",s)
    }
} 

pub enum OperatorError{
    Unknown(char),
    NoChar
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
    type Err=OperatorError;
    type ParseContext=();
    fn try_parse_from_peek<P:crate::try_parse_from_iter::Peek<Item=char>>(peek: &mut P,context:Self::ParseContext)-> Result<Self,Self::Err> {
        let c = peek.peek().cloned().ok_or(OperatorError::NoChar)?;
        let op = Operator::try_from(c);
        if op.is_ok() {
            peek.advance();
        }
        op
    }
}
impl Bracket {
    pub fn open(&self)->char{
        match self{
            Self::Parenthesis => '(',
        }
    }
    pub fn close(&self)->char{
        match self{
            Self::Parenthesis => ')',
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Token {
    Bracket{
        bracket:Bracket,
        state:BracketState,
    },
    Number(isize),
    Operator(Operator),
}
impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bracket {bracket,state}=>write!(f,"[Bracket: {:?}{:?}]",state,bracket),
            Self::Number (n)=> write!(f, "[Number: {:?}]",n),
            Self::Operator(op)=>Debug::fmt(op,f),
        }
    }
}
