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
        write!(f,"[Operator: {}]",s)
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
