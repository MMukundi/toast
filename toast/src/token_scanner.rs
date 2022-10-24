use crate::token::{FileLocation, Token, TokenData};
use crate::try_parse_from_iter::{Peek, TryParseFromPeek};

pub struct TokenScanner<T> {
    lines: T,
    current_line: Option<String>,
}

impl<T> TokenScanner<T> {
    pub fn new(t: T) -> Self {
        Self {
            lines: t,
            current_line: None,
        }
    }
}

impl<T: Iterator<Item = String>> Iterator for TokenScanner<T> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let current_line = {
            match self.current_line {
                Some(ref mut current_line) => {
                    if current_line.is_empty() {
                        self.current_line = self.lines.next();
                        self.current_line.as_mut()?
                    } else {
                        current_line
                    }
                }
                None => {
                    self.current_line = self.lines.next();
                    self.current_line.as_mut()?
                }
            }
        };
        let mut line_chars = current_line.chars().peekable();
        skip_whitespace(&mut line_chars);
        let token = TokenData::try_parse_from_peek(&mut line_chars, ())
            .ok()
            .map(|data| Token::new(FileLocation::new(0, 0), data));
        skip_whitespace(&mut line_chars);
        if token.is_some() {
            *current_line = line_chars.collect::<String>();
        }
        token
    }
}

fn skip_whitespace<P: Peek<Item = char>>(peek: &mut P) {
    peek.peek_while(|c| c.is_whitespace().then_some(()))
        .for_each(drop);
}
#[cfg(test)]
mod tests {
    use crate::token::{Bracket, BracketState, BracketType, Operator, TokenData};

    use super::TokenScanner;

    #[test]
    fn just_nums() {
        assert_eq!(
            TokenScanner::new("1 2 3 4\n 5 6 7 8".lines().map(|s| s.to_string()))
                .map(|t| t.token_data)
                .collect::<Vec<_>>(),
            [1, 2, 3, 4, 5, 6, 7, 8]
                .map(TokenData::Number)
                .into_iter()
                .collect::<Vec<_>>()
        )
    }
    #[test]
    fn just_ops() {
        assert_eq!(
            TokenScanner::new("+ + / -\n * - / %".lines().map(|s| s.to_string()))
                .map(|t| t.token_data)
                .collect::<Vec<_>>(),
            vec![
                TokenData::Operator(Operator::Add),
                TokenData::Operator(Operator::Add),
                TokenData::Operator(Operator::Div),
                TokenData::Operator(Operator::Sub),
                TokenData::Operator(Operator::Mul),
                TokenData::Operator(Operator::Sub),
                TokenData::Operator(Operator::Div),
                TokenData::Operator(Operator::Mod),
            ]
        )
    }

    #[test]
    fn nums_and_ops() {
        assert_eq!(
            TokenScanner::new(
                "1 1 + 2 2 + 3 / 4 -\n * -1 / 1 %"
                    .lines()
                    .map(|s| s.to_string())
            )
            .map(|t| t.token_data)
            .collect::<Vec<_>>(),
            vec![
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
            ]
        )
    }

    #[test]
    fn space_around_newline() {
        assert_eq!(
            TokenScanner::new("1 \n 2".lines().map(|s| s.to_string()))
                .map(|t| t.token_data)
                .collect::<Vec<_>>(),
            vec![TokenData::Number(1), TokenData::Number(2)],
        )
    }

    #[test]
    fn negative_one_and_minus() {
        assert_eq!(
            TokenScanner::new("-1 - ".lines().map(|s| s.to_string()))
                .map(|t| t.token_data)
                .collect::<Vec<_>>(),
            vec![TokenData::Number(-1), TokenData::Operator(Operator::Sub)],
        )
    }

    #[test]
    fn negative_one_and_minus_with_parens() {
        assert_eq!(
            TokenScanner::new("( -1 - ) ".lines().map(|s| s.to_string()))
                .map(|t| t.token_data)
                .collect::<Vec<_>>(),
            vec![
                TokenData::Bracket(Bracket::new(BracketType::Parenthesis, BracketState::Open)),
                TokenData::Number(-1),
                TokenData::Operator(Operator::Sub),
                TokenData::Bracket(Bracket::new(BracketType::Parenthesis, BracketState::Close))
            ],
        )
    }

    #[test]
    fn just_parens() {
        assert_eq!(
            TokenScanner::new("())())()(()()())".lines().map(|s| s.to_string()))
                .map(|t| t.token_data)
                .collect::<Vec<_>>(),
            vec![
                TokenData::Bracket(Bracket::new(BracketType::Parenthesis, BracketState::Open)),
                TokenData::Bracket(Bracket::new(BracketType::Parenthesis, BracketState::Close)),
                TokenData::Bracket(Bracket::new(BracketType::Parenthesis, BracketState::Close)),
                TokenData::Bracket(Bracket::new(BracketType::Parenthesis, BracketState::Open)),
                TokenData::Bracket(Bracket::new(BracketType::Parenthesis, BracketState::Close)),
                TokenData::Bracket(Bracket::new(BracketType::Parenthesis, BracketState::Close)),
                TokenData::Bracket(Bracket::new(BracketType::Parenthesis, BracketState::Open)),
                TokenData::Bracket(Bracket::new(BracketType::Parenthesis, BracketState::Close)),
                TokenData::Bracket(Bracket::new(BracketType::Parenthesis, BracketState::Open)),
                TokenData::Bracket(Bracket::new(BracketType::Parenthesis, BracketState::Open)),
                TokenData::Bracket(Bracket::new(BracketType::Parenthesis, BracketState::Close)),
                TokenData::Bracket(Bracket::new(BracketType::Parenthesis, BracketState::Open)),
                TokenData::Bracket(Bracket::new(BracketType::Parenthesis, BracketState::Close)),
                TokenData::Bracket(Bracket::new(BracketType::Parenthesis, BracketState::Open)),
                TokenData::Bracket(Bracket::new(BracketType::Parenthesis, BracketState::Close)),
                TokenData::Bracket(Bracket::new(BracketType::Parenthesis, BracketState::Close)),
            ],
        )
    }
}
