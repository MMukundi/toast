use std::iter::Peekable;
use crate::expression::{BuiltIn, BuiltInFunction, Call, CodeBlock, Definition, Expression};
use crate::tokens::{Bracket, BracketState, Keyword, Token};
use crate::util::BorrowedFilter;

pub struct  Parser<I:Iterator> {
    parsed: Vec<Expression>,
    tokens: Option<Peekable<I>>,
}
impl <I:Iterator<Item=Token>> Parser<I> {
    pub fn new(tokens:I)->Self{
        Self {
            parsed:Default::default(),
            tokens:Some(tokens.peekable())
        }
    }
    pub fn parse_next_expr(&mut self) ->Option<Expression> {
        let tokens = if let Some(ts) = &mut self.tokens {
            ts
        } else {
            return self.parsed.pop();
        };
        self.parsed.clear();
        loop {
            let next = match tokens.next() {
                Some(e)=>e,
                None =>{
                    self.tokens = None;
                    return self.parsed.pop();
                }
            };
            match next {
                Token::Bracket { state:BracketState::Open,bracket } => {
                    let mut close_missing:bool=true;
                    let mut bracket_stack:Vec<Bracket>=Default::default();
                    let mut between_brackets:Vec<Token>=Default::default();
                    bracket_stack.push(bracket);
                    while let Some(t) = tokens.peek(){
                        match &t {
                            Token::Bracket {bracket,state:BracketState::Close}=> {
                                if bracket_stack.last().map(|s|bracket==s).unwrap_or(false) {
                                    bracket_stack.pop(); // Remove the match
                                }else {
                                    panic!("Unmatched {:?} bracket",bracket);
                                }
                            },
                            Token::Bracket {bracket,state:BracketState::Open}=> {
                                bracket_stack.push(*bracket);
                            }
                            _=>{}
                        };
                        if bracket_stack.is_empty() {
                            close_missing = false;
                            break
                        }
                        tokens.next().map(|n|between_brackets.push(n));
                    }
    
                    let expressions: Vec<_> = Parser::new(between_brackets.into_iter()).collect();
                    if close_missing{
                        panic!("Unclosed {:?} bracket",bracket);
                    } else {
                        tokens.next(); //Consume the close
                    }
                    match bracket {
                        Bracket::Curly => {
                            self.parsed.push(Expression::CodeBlock(CodeBlock {
                                expressions
                            }));
                        },
                        // Bracket::Angle => {},
                        // Bracket::Square => {},
                        // Bracket::Parenthesis => {},
                        _=>panic!("Cannot yet handle this bracket type")
                    }
                },
                Token::Bracket { state:BracketState::Close,bracket } => {
                    panic!("Unmatched {:?} bracket",bracket);
                },
                Token::Literal(l) => {
                    self.parsed.push(Expression::Literal(l));
                },
                Token::Identifier(id) => {
                    self.parsed.push(Expression::Identifier(id));
                },
                Token::Operator(operator) => {
                    let top_argument = self.parsed.pop().expect("Operators need 2 arguments, none provided");
                    let bottom_argument = self.parsed.pop().expect("Operators need 2 arguments, only 1 provided");
                    self.parsed.push(Expression::BuiltInIdentifier(BuiltIn::Function(BuiltInFunction::MathOperator{
                        operator,
                        arguments: Box::new((top_argument,bottom_argument))
                    })));
                },
                Token::Keyword(kw) => {
                    match kw {
                        Keyword::Def => {
                            let identifier_string = if let Expression::Identifier(id_string) = self.parsed.pop().expect("No identifier for definition"){
                                id_string
                            }else{
                                panic!("Cannot use this expression for identifier for definition")
                            };
                            let value = self.parsed.pop().expect("No value to define");
                            break Some(Expression::Definition(Definition{
                                name: identifier_string,
                                value:Box::new(value)
                            }));
                        }
                        Keyword::Call => {
                            let expression_to_call = self.parsed.pop().expect("No expression to call");
                            break Some(Expression::Call(Call{
                                value:Box::new(expression_to_call)
                            }));
                        }
                        Keyword::Print => {
                            self.parsed.push(Expression::BuiltInIdentifier(BuiltIn::Function(BuiltInFunction::Print)));
                        }
                    }
                }
            }    
        }
    }
}

impl <I:Iterator<Item=Token>> Iterator for Parser<I> {
    type Item = Expression;
    fn next(&mut self) -> Option<Self::Item> {
        self.parse_next_expr()
    }
}
