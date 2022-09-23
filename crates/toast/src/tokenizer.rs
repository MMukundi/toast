use std::iter::Peekable;
use std::marker::PhantomData;
use crate::tokens::{Bracket, BracketState, Literal, NumericLiteral, Token};
use crate::util::{BorrowedFilterMap, BorrowedFilter};

pub struct Tokens<I:Iterator> {
    source: Peekable<I>,
}
impl <I:Iterator> Tokens<I>{
    pub fn new(source:I)->Self{
        Self{
            source:source.peekable()
        }
    }
}

#[inline(always)]
fn integrate_digit(radix:u32)->impl FnMut(usize,usize)->usize{
    move |val:usize,digit:usize| val * radix as usize + digit
}

/*
    Reads a string of digits
    Consumes up to but not including the next non-digit character
*/
fn digits<I:Iterator<Item=char>>(peekable_source: &mut Peekable<I>, radix:u32)-> BorrowedFilterMap<I, impl FnMut(&char)->Option<u32>> {
    BorrowedFilterMap::new(peekable_source,move |peeked_char|{
        peeked_char.to_digit(radix)
    })
}
fn parse_integer<I:Iterator<Item=char>>(peekable_source: &mut Peekable<I>, radix:u32,first_digit:Option<u32>)->Option<usize>{
    match first_digit{
        Some(fd) =>{
            Some(digits(peekable_source,radix).map(|x|x as usize).fold(fd as usize,integrate_digit(radix)))
        },
        _=>{
            digits(peekable_source,radix).map(|x|x as usize).reduce(integrate_digit(radix))
        }
    }
}
fn parse_float_or_integer<I:Iterator<Item=char>>(peekable_source: &mut Peekable<I>,radix:u32,first_digit:Option<u32>) ->Option<NumericLiteral>{
    let integer_part = parse_integer(peekable_source,radix,first_digit);
    if let Some('.') = peekable_source.peek() {
        peekable_source.next(); //Consume .
        let (numerator,denominator) = digits(peekable_source,radix).fold((0,1),|(numerator,denominator),next_digit |{
            (numerator*radix as usize+next_digit as usize, denominator*radix as usize)
        });
        Some(NumericLiteral::Decimal{integer_part:integer_part.unwrap_or_default() as isize, fractional_part:(numerator as f64)/(denominator as f64)})
    }else {
        integer_part.map(|x|NumericLiteral::Integer(x as isize))
    }
}
fn consume_radix<I:Iterator<Item=char>>(peekable_source: &mut Peekable<I>) ->Option<u32>{
    match peekable_source.peek() {
        Some('b') => {
            peekable_source.next();
            Some(2)
        },
        Some('o') => {
            peekable_source.next();
            Some(8)
        },
        Some('x') => {
            peekable_source.next();
            Some(16)
        },
        Some(c) if c.is_digit(10) => {
            Some(10)
        },
        _=>None
    }
}


fn parse_positive_number<I:Iterator<Item=char>>(peekable_source: &mut Peekable<I>) ->Option<NumericLiteral>{
    let first_char = peekable_source.peek();
    if let Some(&'0') = first_char { // 0x01FA7E, 0o04713 ,0b0010110, 01239, 0x01FA7E.E, 0o04713.7 ,0b0010110.00100, 01234.91
        peekable_source.next(); // Consume negation
        let radix = consume_radix(peekable_source);
        let value = radix.and_then(|r|parse_float_or_integer(peekable_source,r,None));
        Some(value.unwrap_or(NumericLiteral::Integer(0)))
    } else if let Some(digit) = first_char.and_then(|c|c.to_digit(10))  {
        parse_float_or_integer(peekable_source,10,Some(digit))
    }else{
        None
    }
}

macro_rules! bracket {
    ($self:expr,$bracket:expr,$state:expr) => {
        {
            $self.consume_current_char(); //Consume bracket
            Some(Token::Bracket {bracket:$bracket,state:$state})
        }
    };
}
impl <I:Iterator<Item=char>>Tokens<I> {
    #[inline(always)]
    pub fn consume_current_char(&mut self){
        self.source.next();
    }
}
impl <I:Iterator<Item=char>>Iterator for Tokens<I>{
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
          if let Some(c) = self.source.peek() {
              if !c.is_whitespace() {
                  break;
              }
              self.source.next();
          }  else {
              break;
          }
        };
        match self.source.peek()? {
            '{' => bracket!(self,Bracket::Curly,BracketState::Open),
            '}' => bracket!(self,Bracket::Curly,BracketState::Close),

            '[' => bracket!(self,Bracket::Square,BracketState::Open),
            ']' => bracket!(self,Bracket::Square,BracketState::Close),

            '<' => bracket!(self,Bracket::Angle,BracketState::Open),
            '>' => bracket!(self,Bracket::Angle,BracketState::Close),

            '(' => bracket!(self,Bracket::Parenthesis,BracketState::Open),
            ')' => bracket!(self,Bracket::Parenthesis,BracketState::Close),

            c => {
                let c = *c;
                if let Some(num) = parse_positive_number(&mut self.source) { // 0x01FA7E, 0o04713 ,0b0010110, 01239, 0x01FA7E.E, 0o04713.7 ,0b0010110.00100, 01234.91
                    Some(Token::Literal(Literal::Number(num)))
                }
                else if c == '-' {
                    self.consume_current_char(); // Consume negation
                    if let Some(num) = parse_positive_number(&mut self.source){ // -0x01FA7E, -0o04713 ,-0b0010110, -01239, -0x01FA7E.E, -0o04713.7 ,-0b0010110.00100, -01234.91
                        Some(Token::Literal(Literal::Number(num.negated())))
                    } else{
                        unreachable!("Unexpected negative!");
                    }
                }
                else if c.is_alphabetic() {
                    let ident_iter = BorrowedFilter::new(&mut self.source,|c|c.is_alphanumeric());
                    Some(Token::Identifier(ident_iter.collect()))
                }
                else if c=='\"' {
                    self.consume_current_char(); // Consume quote
                    let mut was_escaped = false;
                    let mut quote_closed = false;
                    let string_literal_iter = BorrowedFilterMap::new(&mut self.source,|c| {
                        if was_escaped {
                            was_escaped =false;
                            Some(Some(match c{
                                'n'=>'\n',
                                't'=>'\t',
                                'r'=>'\r',
                                '"'=>'"',
                                '\''=>'\'',
                                _=>panic!("Invalid escape char"),
                            }))
                        } else if c==&'"'{
                            quote_closed = true;
                            None
                        } else if c==&'\\'{
                            was_escaped =false;
                            Some(None)
                        }else {
                            Some(Some(*c))
                        }
                    }).flatten();
                    let s = Some(Token::Literal(Literal::String(string_literal_iter.collect())));
                    if quote_closed {
                        self.source.next(); // Consume end quote
                    }
                    s
                } else {
                    unreachable!("Unexpected char '{c}'");
                }
            }
            // _ => {
            //     unreachable!("Unexpected token");
            // }
        }
    }
}