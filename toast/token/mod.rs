use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

use crate::numeric_literal::NumericLiteral;

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
        write!(f,"[Op: {}]",s)
    }
} 

pub struct UnknownOperator(char);
impl TryFrom<char> for Operator {
    type Error = UnknownOperator;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '-' => Ok(Self::Sub),
            '*' => Ok(Self::Mul),
            '+' => Ok(Self::Add),
            '/' => Ok(Self::Div),
            '%' => Ok(Self::Div),
            _ => Err(UnknownOperator(value)),
        }
    }
}
impl Bracket {
    pub fn open(&self)->char{
        match self{
            Self::Angle => '<',
            Self::Curly => '{',
            Self::Square => '[',
            Self::Parenthesis => '(',
        }
    }
    pub fn close(&self)->char{
        match self{
            Self::Angle => '>',
            Self::Curly => '}',
            Self::Square => ']',
            Self::Parenthesis => ')',
        }
    }
}
#[derive(Debug,Copy, Clone,Eq, PartialEq)]
pub enum Keyword {
    Def,
    Call,
    Print
}
#[derive(Debug, Clone,Eq, PartialEq)]
pub struct UnknownKeyword(pub String);
impl FromStr for Keyword {
    type Err = UnknownKeyword;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "print" => Ok(Self::Print),
            "call" => Ok(Self::Call),
            "def" => Ok(Self::Def),
            _ => Err(UnknownKeyword(s.to_string())),
        }
    }
}



#[derive(Clone, PartialEq)]
pub enum Literal {
    Number(NumericLiteral),
}
impl Debug for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Number(n) => Debug::fmt(n,f),
        }
    }
}
#[derive(Clone, PartialEq)]
pub enum Token {
    Bracket{
        bracket:Bracket,
        state:BracketState,
    },
    Literal(Literal),
    Identifier(String),
    Keyword(Keyword),
    Operator(Operator),
}
impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bracket {bracket,state}=>write!(f,"<{:?}{:?}>",state,bracket),
            Self::Literal (l)=> Debug::fmt(l,f),
            Self::Identifier(id)=>write!(f, "<[ID]{:?}>",id),
            Self::Keyword(kw)=>write!(f, "<[KW]{:?}>",kw),
            Self::Operator(op)=>Debug::fmt(op,f),
        }
    }
}
