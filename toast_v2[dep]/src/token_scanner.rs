use std::fmt::Debug;
use std::iter::Peekable;

use crate::or::Or;
use crate::parse::parser::{while_match, Parser, ParserOutput, DigitParser, TupleOfParsers, MaybeParser, maybe, ConstCharParser, ManyParser, HexDigitParser, OctalDigitParser};
use crate::stringy::{Poppable, StringyChars};
use crate::token::{ParseTokenErr, SourceLocation, Token, TokenData};
use crate::try_parse_from_iter::{Counted, Peek, TryParseFromPeek};

pub struct TokenScanner<T> {
    lines: T,
    line_counter: usize,
    column_counter: usize,
    current_line: Option<String>,
}


impl<T> TokenScanner<T> {
    pub fn new(t: T) -> Self {
        Self {
            lines: t,
            current_line: None,
            line_counter: 0,
            column_counter: 0,
        }
    }
}

fn get_next_line<'a, 'b,'s, T: Iterator>(
    lines: &'a mut T,
    current_line: &'a mut Option<T::Item>,
    line_counter: &'b mut usize,
    column_counter: &'b mut usize,
) -> Option<&'a mut T::Item> {
    *line_counter += 1;
    *column_counter = 0;
    *current_line = lines
        .next()
        .map(|line| line);
    current_line.as_mut()
}

#[derive(Debug,Clone, Copy,PartialEq, Eq,PartialOrd, Ord)]
pub struct Integer<const BASE:usize>(usize);
#[derive(Debug,Clone, Copy,PartialEq, Eq)]
pub struct Fractional<const BASE:usize>(usize,usize);
impl <const BASE:usize> FromIterator<u8> for Integer<BASE> {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Self(iter.into_iter().fold(0, |value,a|{
            value*BASE + a as usize
        }))
    }
}
impl <const BASE:usize> FromIterator<u8> for Fractional<BASE> {
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        let (n,d) = iter.into_iter().fold((0,1), |(n,d),a|{
            (n*BASE + a as usize,d*BASE)
        });
        Self(n,d)
    }
}

impl <const BASE:usize> Extend<u8> for Integer<BASE> {
    fn extend<T: IntoIterator<Item = u8>>(&mut self,iter: T) {
        for digit in iter {
            self.0 *= BASE;
            self.0 += digit as usize;
        }
    }
}
impl <const BASE:usize> Extend<u8> for Fractional<BASE> {
    fn extend<T: IntoIterator<Item = u8>>(&mut self,iter: T) {
        for digit in iter {
            self.0 *= BASE;
            self.0 += digit as usize;
            self.1 *= BASE;
        }
    }
}

#[derive(Debug,Default,Clone, Copy,PartialEq, Eq)]
pub struct NumberParser<const BASE:usize,D>(D);
fn number<const BASE:usize,D>(digit:D)->NumberParser<BASE,D>{
    NumberParser(digit)
}
impl <'s,const BASE:usize,D:Parser<'s,Output = u8>+Clone> Parser<'s> for NumberParser<BASE,D>{
    type Output=(usize,Option<(usize,usize)>);

    type Error=crate::parse::parser::WithLocation<()>;

    fn parse_next(
        &mut self,
        source: &'s str,
    ) -> Result<ParserOutput<'s, Self::Output>, Self::Error> {
        if let Ok(first_digit) = self.0.parse_next(source){
            (( self.0.clone()).many_with::<Integer<BASE>>(Integer(first_digit.output as _)),maybe((ConstCharParser::<'.'>,(&mut self.0).many::<Fractional<BASE>>()))).map(|((int,_),maybe_frac)|{
                (int.0,if let Some(((),(Fractional(n, d),_)))=maybe_frac{
                    Some((n,d))
                }else{
                    None
                })
            }).parse_next(first_digit.remaining).map(|out|out.with_offset(first_digit.chars_consumed)).map_err(|e|e.map(|_|{}))
        }else {
            (ConstCharParser::<'.'>,(&mut self.0).at_least_one::<Fractional<BASE>>()).map(|(_,(frac,_))|{
                (0,Some((frac.0,frac.1)))
            }).parse_next(source).map_err(|e|e.map(|_|{}))
        }
    }
}

impl<T: Iterator<Item = String>> Iterator for TokenScanner<T> {
    type Item = Result<Token, ParseTokenErr>;

    fn next(&mut self) -> Option<Self::Item> {
        let current_line = {
            match self.current_line {
                Some(ref mut current_line) => {
                    if current_line.is_empty() {
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

        // line_chars.
        // let current_position = line_chars.l;
        skip_whitespace(line_chars,&mut self.line_counter, &mut self.column_counter);





        // let ParserOutput{
        //     output,
        //     remaining,
        //     chars_consumed
        // } = ;


        // let token = TokenData::try_parse_from_peek(line_chars, ())
        //     // .ok()
        //     .map(|data| {
        //         let end_position = line_chars.items_yielded();
        //         Token::new(
        //             SourceLocation::new(
        //                 self.line_counter,
        //                 current_position + 1,
        //                 end_position - current_position,
        //             ),
        //             data,
        //         )
        //     });
        // skip_whitespace(&mut line_chars);
        // if token.is_err() {
        //     line_chars.next(); // Consume the offending char
        // }
        // Some(token)
        None
    }
}

fn skip_whitespace<S:AsRef<str>+for<'s>From<&'s str>>(s: &mut S,row:&mut usize,col:&mut usize) {
    match while_match(char::is_whitespace).parse_next(s.as_ref()) {
        Ok(ParserOutput { output, remaining, chars_consumed }) => {
            let lines = output.lines().count();
            if lines > 0 {
                *row += lines;
                *col = 0;
            }else{
                *col+=chars_consumed;
            }
            *s = remaining.into();
        },
        Err(err) => match err {},
    }
}

#[cfg(test)]
mod tests {
    use core::num;

    use crate::or::{Or, CollapseRight, CollapseLeft};
    use crate::parse::parser::{ConstCharParser, maybe, HexDigitParser, OctalDigitParser, DigitParser, TupleOfParsers, Parser};
    use crate::stringy::Poppable;
    use crate::token::{Bracket, BracketType, IdentifierLike, Operator, ParseTokenErr, TokenData};

    use super::{TokenScanner, number};

    fn get_tokens<T: IntoIterator<Item = String>>(
        iter: T,
    ) -> Result<Vec<TokenData>, ParseTokenErr> {
        TokenScanner::new(iter.into_iter())
            .map(|t| t.map(|inner| inner.token_data))
            .collect::<Result<Vec<TokenData>, ParseTokenErr>>()
    }

    #[test]
    fn just_nums() {
        assert_eq!(
            get_tokens("1 2 3 4\n 5 6 7 8".lines().map(ToOwned::to_owned)),
            Ok([1, 2, 3, 4, 5, 6, 7, 8]
                .map(TokenData::from)
                .into_iter()
                .collect::<Vec<_>>())
        )
    }
    #[test]
    fn just_ops() {
        assert_eq!(
            get_tokens("+ + / -\n * - / %".lines().map(ToOwned::to_owned)),
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
            get_tokens("1 1 + 2 2 + 3 / 4 -\n * -1 / 1 %".lines().map(ToOwned::to_owned)),
            Ok(vec![
                TokenData::from(1),
                TokenData::from(1),
                TokenData::Operator(Operator::Add),
                TokenData::from(2),
                TokenData::from(2),
                TokenData::Operator(Operator::Add),
                TokenData::from(3),
                TokenData::Operator(Operator::Div),
                TokenData::from(4),
                TokenData::Operator(Operator::Sub),
                TokenData::Operator(Operator::Mul),
                TokenData::from(-1),
                TokenData::Operator(Operator::Div),
                TokenData::from(1),
                TokenData::Operator(Operator::Mod),
            ])
        )
    }

    #[test]
    fn space_around_newline() {
        assert_eq!(
            get_tokens("1 \n 2".lines().map(ToOwned::to_owned)),
            Ok(vec![TokenData::from(1), TokenData::from(2)]),
        )
    }

    #[test]
    fn negative_one_and_minus() {
        assert_eq!(
            get_tokens("-1 - ".lines().map(ToOwned::to_owned)),
            Ok(vec![
                TokenData::from(-1),
                TokenData::Operator(Operator::Sub)
            ])
        )
    }

    #[test]
    fn negative_one_and_minus_with_parens() {
        assert_eq!(
            get_tokens("( -1 - ) ".lines().map(ToOwned::to_owned)),
            Ok(vec![
                TokenData::Bracket(Bracket::Open(BracketType::Parenthesis)),
                TokenData::from(-1),
                TokenData::Operator(Operator::Sub),
                TokenData::Bracket(Bracket::Close(BracketType::Parenthesis))
            ]),
        )
    }

    #[test]
    fn just_parens() {
        assert_eq!(
            get_tokens("())())()(()()())".lines().map(ToOwned::to_owned)),
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
        assert_eq!(get_tokens("  ".lines().map(ToOwned::to_owned)), Err(ParseTokenErr::Empty),)
    }
    #[test]
    fn identifier() {
        assert_eq!(
            get_tokens("not_a_keyword35".lines().map(ToOwned::to_owned)),
            Ok(vec![TokenData::Identifier(
                "not_a_keyword35".to_string().into()
            )]),
        )
    }
    #[test]
    fn identifier_read_as_siffix() {
        use crate::token::NumericLiteral;
        assert_eq!(
            get_tokens("35not_a_keyword".lines().map(ToOwned::to_owned)),
            Ok(vec![TokenData::Number(NumericLiteral::new(
                35,
                Some("not_a_keyword".to_string())
            ))]),
        )
    }

    #[test]
    fn number_parser(){
        let bit_parser = (ConstCharParser::<'0'>,ConstCharParser::<'1'>).one_of().map(|bit|match bit {
            Or::Left(_)=>0,
            Or::Right(_)=>1
        });
        let mut number_parser = (
            // Maybe negative
            maybe(ConstCharParser::<'-'>),
            // And then either 
            (
                // A prefixed number:
                (ConstCharParser::<'0'>,
                    (
                        (ConstCharParser::<'b'>,number::<2,_>(bit_parser)).map(|out|out.1),
                        (ConstCharParser::<'x'>,number::<16,_>(HexDigitParser)).map(|out|out.1),
                        (ConstCharParser::<'o'>,number::<8,_>(OctalDigitParser)).map(|out|out.1),
                        number::<10,_>(DigitParser),
                    ).one_of()
                ).map(|out|out.1),
                // or just a decimal number
                number::<10,_>(DigitParser),
            ).one_of(),
        ).map(|out|{
            let (int,frac)=CollapseRight::<(usize, Option<(usize, usize)>)>::collapse_right(out.1.flip());
            (out.0.is_some(),int,frac)
        });
        for num in [
            "0xFF.FF",
            "0o77.77",
            "0b11.11",
            "099.99",
            "99.99"
        ] {
            println!("Parsing {num} => {:?}",number_parser.parse_next(num))
        }
        for num in [
            "-0xFF.FF",
            "-0o77.77",
            "-0b11.11",
            "-099.99",
            "-99.99"
        ] {
            println!("Parsing {num} => {:?}",number_parser.parse_next(num))
        }
        for num in [
            "--0xFF.FF",
            "--0o77.77",
            "--0b11.11",
            "--099.99",
            "--99.99"
        ] {
            println!("Parsing {num} => {:?}",number_parser.parse_next(num))
        }
    }
}
