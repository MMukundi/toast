extern crate core;
use crate::parser::Parser;
use crate::tokens::Token;

mod tokenizer;
mod tokens;
mod util;
mod parser;
mod expression;

fn main() {
    // let s = "({)-313456758765{34}";
    // let tokens:Vec<_> = tokenizer::Tokens::new(s.chars()).collect();
    // println!("Hello, world! Here are the tokens of {s}:\n{:?}\n",tokens);
    let s = "{ 43 life def } def_life def";
    let tokens:Vec<_> = tokenizer::Tokens::new(s.chars()).collect();
    println!("Hello, world! Here are the tokens of {s}:\n{:?}\n",&tokens);
    let exprs:Parser<<Vec<Token> as IntoIterator>::IntoIter> = Parser::new(tokens.into_iter());
    println!("Hello, world! Here are the expressions of {s}:\n{:?}\n",exprs.flatten().collect::<Vec<_>>());
}
