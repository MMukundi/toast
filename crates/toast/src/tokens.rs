use std::fmt::{Debug, Formatter, write};
use std::str::FromStr;

#[derive(Debug,Copy, Clone,Eq, PartialEq)]
pub enum BracketState {
    Open,
    Close
}
#[derive(Debug,Copy, Clone,Eq, PartialEq)]
pub enum Bracket {
    Angle,
    Curly,
    Square,
    Parenthesis,
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


#[derive(Debug, Clone,Copy, PartialEq,Eq)]
pub enum Sign {
    Positive,
    Negative
}
impl Sign {
    pub fn flip(&mut self) {
        *self = match self {
            Self::Positive => Self::Negative,
            Self::Negative => Self::Positive
        };
    }
}

#[derive(Clone, PartialEq)]
pub enum NumericLiteral {
    Integer(isize),
    Decimal{
        sign:Sign,
        integer_part:usize,
        fraction_numerator:usize,
        fraction_denominator:usize,
    },
}
impl Debug for NumericLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NumericLiteral::Integer(v) => write!(f, "<[Integer]{v}>"),
            NumericLiteral::Decimal { sign,integer_part,fraction_numerator,fraction_denominator } => {
                write!(f,"<[Float]{}{integer_part} {fraction_numerator}/{fraction_denominator}>",if *sign==Sign::Negative {"-"}else{""})
            }
        }
    }
}
impl NumericLiteral{
    pub fn negate(&mut self){
        match self {
            Self::Decimal {sign,..}=>sign.flip(),
            Self::Integer(value)=>{*value*=-1;}
        };
    }
}
#[derive(Clone, PartialEq)]
pub enum Literal {
    Number(NumericLiteral),
    String(String),
}
impl Debug for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Number(n) => Debug::fmt(n,f),
            Literal::String(s) => write!(f,"<[STR, {}]{:?}>",s.len(),s)
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
}
impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bracket {bracket,state}=>write!(f,"<{:?}{:?}>",state,bracket),
            Self::Literal (l)=> Debug::fmt(l,f),
            Self::Identifier(id)=>write!(f, "<[ID]{:?}>",id),
            Self::Keyword(kw)=>write!(f, "<[KW]{:?}>",kw),
        }
    }
}
