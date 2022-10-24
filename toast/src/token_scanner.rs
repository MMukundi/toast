use std::borrow::BorrowMut;
use std::{ops::Deref, iter::Peekable, str::Chars};

use crate::token::{Token, Operator};
use crate::try_parse_from_iter::{TryParseFromPeek, Peek};

pub struct TokenScanner<T>{
    lines: T,
    current_line:Option<String>
}

impl <T> TokenScanner<T>{
    fn new(t:T)->Self{
        Self { lines: t, current_line: None }
    }
}

impl <T:Iterator<Item=String>> Iterator for TokenScanner<T>{
    type Item=Token;

    fn next(&mut self) -> Option<Self::Item> {
        let current_line = {
            match self.current_line {
                Some(ref mut current_line)=>{
                    if current_line.is_empty() {
                        self.current_line = self.lines.next();
                        self.current_line.as_mut()?
                    }else{
                        current_line
                    }
                },
                None => {
                    self.current_line = self.lines.next();
                    self.current_line.as_mut()?
                }
            }
        };
        let mut line_chars = current_line.chars().peekable();
        skip_whitespace(&mut line_chars);
        let token = try_parse_next_token(&mut line_chars);
        skip_whitespace(&mut line_chars);
        if token.is_some(){
            *current_line = line_chars.collect::<String>();
        }
        token
    }
}

fn skip_whitespace<P:Peek<Item = char>>(peek:&mut P){
    peek.peek_while(|c|c.is_whitespace().then_some(())).for_each(drop);
}

fn try_parse_next_token<P:Peek<Item = char>+Clone>(line:&mut P)-> Option<Token>{
    let save_state = line.clone();
    let parsed_num = isize::try_parse_from_peek(
         line,
        10
    );
    if let Ok(n) = parsed_num {
        return Some(Token::Number(n));
    };
    *line = save_state;
    let save_state = line.clone();
    let parsed_op = Operator::try_parse_from_peek(line, ());
    if let Ok(op) = parsed_op {
        return Some(Token::Operator(op));
    };
    *line = save_state;
    None
}

#[cfg(test)]
mod tests {
    use crate::token::{Token, Operator};

    use super::TokenScanner;

    #[test]
    fn just_nums(){
        assert_eq!(
            TokenScanner::new("1 2 3 4\n 5 6 7 8".lines().map(|s|s.to_string())).collect::<Vec<_>>(),
            [1,2,3,4,5,6,7,8].map(Token::Number).into_iter().collect::<Vec<_>>()
        )
    }
    #[test]
    fn just_ops(){
        assert_eq!(
            TokenScanner::new("+ + / -\n * - / %".lines().map(|s|s.to_string())).collect::<Vec<_>>(),
            vec![
                Token::Operator(Operator::Add), Token::Operator(Operator::Add),Token::Operator(Operator::Div),
                Token::Operator(Operator::Sub),Token::Operator(Operator::Mul),Token::Operator(Operator::Sub),
                 Token::Operator(Operator::Div),
                Token::Operator(Operator::Mod),
            ]
        )
    }

    #[test]
    fn nums_and_ops(){
        assert_eq!(
            TokenScanner::new("1 1 + 2 2 + 3 / 4 -\n * -1 / 1 %".lines().map(|s|s.to_string())).collect::<Vec<_>>(),
            vec![
                Token::Number(1),Token::Number(1), Token::Operator(Operator::Add),
                Token::Number(2),Token::Number(2), Token::Operator(Operator::Add),
                Token::Number(3), Token::Operator(Operator::Div),Token::Number(4),
                Token::Operator(Operator::Sub),
                Token::Operator(Operator::Mul),
                Token::Number(-1), Token::Operator(Operator::Div),Token::Number(1),
                Token::Operator(Operator::Mod),
            ]
        )
    }
    
    #[test]
    fn space_around_newline(){
        assert_eq!(
            TokenScanner::new("1 \n 2".lines().map(|s|s.to_string())).collect::<Vec<_>>(),
            vec![Token::Number(1),Token::Number(2)],
        )
    }
       
    #[test]
    fn negative_one_and_minus(){
        assert_eq!(
            TokenScanner::new("-1 - ".lines().map(|s|s.to_string())).collect::<Vec<_>>(),
            vec![Token::Number(-1),Token::Operator(Operator::Sub)],
        )
    }
}