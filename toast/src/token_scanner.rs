use std::iter::{Peekable};


use crate::stringy::{Poppable, StringyChars};
use crate::token::{SourceLocation, Token, TokenData, ParseTokenErr};
use crate::try_parse_from_iter::{Peek, TryParseFromPeek, Counted};

pub struct TokenScanner<T, S:Poppable> {
    lines: T,
    line_counter: usize,
    column_counter: usize,
    current_line: Option<Counted<Peekable<StringyChars<S>>>>,
}

impl<T, S: Poppable> TokenScanner<T, S> {
    pub fn new(t: T) -> Self {
        Self {
            lines: t,
            current_line: None,
            line_counter: 0,
            column_counter: 0,
        }
    }
}

fn get_next_line<'a, 'b, T: Iterator<Item = S>, S: Poppable>(
    lines: &'a mut T,
    current_line: &'a mut  Option<Counted<Peekable<StringyChars<S>>>>,
    line_counter: &'b mut usize,
    column_counter: &'b mut usize,
) ->  Option<&'a mut Counted<Peekable<StringyChars<S>>>> {
    *line_counter += 1;
    *column_counter = 0;
    *current_line = lines.next().map(|line| Counted::new(StringyChars::new(line).peekable()));
    current_line.as_mut()
}


impl<T: Iterator<Item = S>,S:Poppable> Iterator for TokenScanner<T, S>
{
    type Item = Result<Token,ParseTokenErr>;

    fn next(&mut self) -> Option<Self::Item> {
        let current_line = {
            match self.current_line {
                Some(ref mut current_line) => {
                    if current_line.peek().is_none() {
                        get_next_line(
                            &mut self.lines,
                            &mut self.current_line,
                            &mut self.line_counter,
                            &mut self.column_counter,
                        )
                    } else {
                        Some(current_line)
                    }
                }
                None => get_next_line(
                    &mut self.lines,
                    &mut self.current_line,
                    &mut self.line_counter,
                    &mut self.column_counter,
                ),
            }
        };
        let mut line_chars = current_line?;
        let current_position= line_chars.index();
        skip_whitespace(&mut line_chars);
        let token = TokenData::try_parse_from_peek(line_chars, ())
            // .ok()
            .map(|data| {
                let end_position= line_chars.index();
                Token::new(SourceLocation::new(self.line_counter, current_position+1,end_position-current_position), data)
            });
        skip_whitespace(&mut line_chars);
        if token.is_err() {
            line_chars.next(); // Consume the offending char
        }
        Some(token)
    }
}

fn skip_whitespace<P: Peek<Item = char>>(peek: &mut P) {
    peek.peek_while(|c| c.is_whitespace().then_some(()))
        .for_each(drop);
}

#[cfg(test)]
mod tests {
    use crate::stringy::Poppable;
    use crate::token::{Bracket, BracketType, Operator, TokenData, ParseTokenErr};

    use super::TokenScanner;

    fn get_tokens<T: IntoIterator<Item = S>,S:Poppable>(iter:T)->Result<Vec<TokenData>,ParseTokenErr> {
        TokenScanner::new(iter.into_iter())
            .map(|t| t.map(|inner|inner.token_data))
            .collect::<Result<Vec<TokenData>,ParseTokenErr>>()
    }

    #[test]
    fn just_nums() {
        assert_eq!(
            get_tokens("1 2 3 4\n 5 6 7 8".lines()),
            Ok([1, 2, 3, 4, 5, 6, 7, 8]
                .map(TokenData::Number)
                .into_iter()
                .collect::<Vec<_>>())
        )
    }
    #[test]
    fn just_ops() {
        assert_eq!(
            get_tokens("+ + / -\n * - / %".lines()),
            Ok(vec![
                TokenData::Operator(Operator::Add),
                TokenData::Operator(Operator::Add),
                TokenData::Operator(Operator::Div),
                TokenData::Operator(Operator::Sub),
                TokenData::Operator(Operator::Mul),
                TokenData::Operator(Operator::Sub),
                TokenData::Operator(Operator::Div),
                TokenData::Operator(Operator::Mod),
            ])
        )
    }

    #[test]
    fn nums_and_ops() {
        assert_eq!(
            get_tokens(
                "1 1 + 2 2 + 3 / 4 -\n * -1 / 1 %"
                    .lines()
            ),
            Ok(vec![
                TokenData::Number(1),
                TokenData::Number(1),
                TokenData::Operator(Operator::Add),
                TokenData::Number(2),
                TokenData::Number(2),
                TokenData::Operator(Operator::Add),
                TokenData::Number(3),
                TokenData::Operator(Operator::Div),
                TokenData::Number(4),
                TokenData::Operator(Operator::Sub),
                TokenData::Operator(Operator::Mul),
                TokenData::Number(-1),
                TokenData::Operator(Operator::Div),
                TokenData::Number(1),
                TokenData::Operator(Operator::Mod),
            ])
        )
    }

    #[test]
    fn space_around_newline() {
        assert_eq!(
            get_tokens("1 \n 2".lines()),
            Ok(vec![TokenData::Number(1), TokenData::Number(2)]),
        )
    }

    #[test]
    fn negative_one_and_minus() {
        assert_eq!(
            get_tokens("-1 - ".lines()),
            Ok(vec![TokenData::Number(-1), TokenData::Operator(Operator::Sub)])
        )
    }

    #[test]
    fn negative_one_and_minus_with_parens() {
        assert_eq!(
            get_tokens("( -1 - ) ".lines()),
            Ok(vec![
                TokenData::Bracket(Bracket::Open(BracketType::Parenthesis)),
                TokenData::Number(-1),
                TokenData::Operator(Operator::Sub),
                TokenData::Bracket(Bracket::Close(BracketType::Parenthesis))
            ]),
        )
    }

    #[test]
    fn just_parens() {
        assert_eq!(
            get_tokens("())())()(()()())".lines()),
            Ok(vec![
                TokenData::Bracket(Bracket::Open(BracketType::Parenthesis)),
                TokenData::Bracket(Bracket::Close(BracketType::Parenthesis)),
                TokenData::Bracket(Bracket::Close(BracketType::Parenthesis)),
                TokenData::Bracket(Bracket::Open(BracketType::Parenthesis)),
                TokenData::Bracket(Bracket::Close(BracketType::Parenthesis)),
                TokenData::Bracket(Bracket::Close(BracketType::Parenthesis)),
                TokenData::Bracket(Bracket::Open(BracketType::Parenthesis)),
                TokenData::Bracket(Bracket::Close(BracketType::Parenthesis)),
                TokenData::Bracket(Bracket::Open(BracketType::Parenthesis)),
                TokenData::Bracket(Bracket::Open(BracketType::Parenthesis)),
                TokenData::Bracket(Bracket::Close(BracketType::Parenthesis)),
                TokenData::Bracket(Bracket::Open(BracketType::Parenthesis)),
                TokenData::Bracket(Bracket::Close(BracketType::Parenthesis)),
                TokenData::Bracket(Bracket::Open(BracketType::Parenthesis)),
                TokenData::Bracket(Bracket::Close(BracketType::Parenthesis)),
                TokenData::Bracket(Bracket::Close(BracketType::Parenthesis)),
            ]),
        )
    }
    #[test]
    fn empty() {
        assert_eq!(
            get_tokens("  ".lines()),
            Err(ParseTokenErr::Empty),
        )
    }
    #[test]
    fn invalid() {
        assert_eq!(
            get_tokens("not_a_keyword".lines()),
            Err(ParseTokenErr::UnexpectedCharacter('n')),
        )
    }
}
