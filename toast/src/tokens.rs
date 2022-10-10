use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Neg,Mul,Sub,Div,Rem};
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
        write!(f,"<[OP]{}>",s)
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
pub struct NumericLiteral {
    pub sign:Sign,
    pub integer_part:usize,
    pub fractional_part:Option<(usize,usize)>,
}

impl NumericLiteral {
    pub fn integer(integer:isize) -> Self {
        if integer < 0 {
            Self {
                sign: Sign::Negative,
                integer_part: -integer as _,
                fractional_part: None
            }
        }else {
            Self {
                sign: Sign::Positive,
                integer_part: integer as _,
                fractional_part: None
            }
        }
    }
}

impl From<NumericLiteral> for f64 {
    fn from(NumericLiteral{sign,integer_part,fractional_part}: NumericLiteral) -> Self {
        let mut f = (integer_part as Self) + fractional_part.map(|(n,d)|(n as Self)/(d as Self)).unwrap_or_default();
        if sign == Sign::Negative {
            f=f.neg()
        };
        f
    }
}
impl Add for NumericLiteral {
    type Output = NumericLiteral;

    fn add(self, rhs: Self) -> Self::Output {
        if self.sign==rhs.sign {
            todo!();
            // Self {
            //     sign: self.sign,
            //     integer_part: self.integer_part + rhs.integer_part,
            //     fractional_part: self.fractional_part.zip(rhs.fractional_part).map(|(l0,l1),(r0,r1)|{
            //         (l0*r1+l1*r0 ,l1*r1)
            //     })
            // }
        } else {
            let int_diff = self.integer_part - rhs.integer_part;
            let frac_parts = match (self.fractional_part,rhs.fractional_part) {
                (Some((a,b)),None) => Some(((a,0),b)),
                (None,Some((a,b))) => Some(((0,a),b)),
                (Some((a,b)),Some((c,d))) => Some(((a*d,c*b),b*d)),
                _ => None
            }.map(|((n1,n2),d)|{
                (n1.cmp(&n2),n1.abs_diff(n2),d)
            });


            todo!();
            // let a  =match 0.cmp(int_diff) {
            //     Ordering::Less => {}
            //     Ordering::Equal => {}
            //     Ordering::Greater => {}
            // };

            // let self_frac = (self.fractional_part.0*rhs.fractional_part.1,self.fractional_part.1*rhs.fractional_part.1);
            // let frac_diff = self.fractional_part.0 - rhs.fractional_part.0;
            // let (sign) = if int_diff > 0 {
            //     self.sign
            // } else {
            //     rhs.sign
            // };
            // Self {
            //     sign: self.sign,
            //     integer_part: self.integer_part + rhs.integer_part,
            //     fractional_part: self.fractional_part.zip(rhs.fractional_part).map(|(l0,l1),(r0,r1)|{
            //         (l0*r1+l1*r0 ,l1*r1)
            //     })
            // }
        }
    }
}
impl Mul for NumericLiteral {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output{todo!()}
}
impl Div for NumericLiteral {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output{todo!()}
}
impl Sub for NumericLiteral {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output{todo!()}
}
impl Rem for NumericLiteral {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output{todo!()}
}

impl Debug for NumericLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.sign == Sign::Negative {
            write!(f, "-")?;
        }
        write!(f,"{}",self.integer_part)?;
        if let Some((n,d)) = self.fractional_part {
            write!(f," {}/{}",n,d)?;
        }
        Ok(())
    }
}
impl NumericLiteral{
    pub fn negate(&mut self){
        self.sign.flip()
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
