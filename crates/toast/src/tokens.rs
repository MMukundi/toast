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
#[derive(Debug, Clone, PartialEq)]
pub enum NumericLiteral {
    Integer(isize),
    Decimal{
        integer_part:isize,
        fractional_part:f64
    },
}
impl NumericLiteral{
    pub fn integer_part(&mut self)->&mut isize{
        match self {
            Self::Integer(i)=> i,
            Self::Decimal { integer_part, .. } => integer_part
        }
    }
    pub fn negated(mut self)->Self{
        let ip = self.integer_part();
        *ip *= -1;
        self
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Number(NumericLiteral),
    String(String),
}
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Bracket{
        bracket:Bracket,
        state:BracketState,
    },
    Literal(Literal),
    Identifier(String),
}