extern crate core;

use std::io;
use std::io::{BufRead, Read, Write};
use crate::codegen::Backend;
use crate::codegen::interpreter::Interpreter;
use crate::parser::Parser;
use crate::tokens::Token;

mod tokenizer;
mod tokens;
mod util;
mod parser;
mod expression;
mod codegen;
mod numeric_literal;

fn main() {
    // let mut stdin = std::io::stdin().lock();
    let mut buffer = String::default();
    let interpreter_input = std::iter::from_fn(||{
        buffer.clear();
        print!(">> ");
        io::stdout().flush().expect("Flushing error");
        std::io::stdin().read_line(&mut buffer).expect("Error reading input");
        Some(buffer.chars().collect::<Vec<_>>().into_iter().chain(std::iter::once('\n')))
    }).flatten();
    let tokens = tokenizer::Tokens::new(interpreter_input);
    let expressions = Parser::new(tokens);
    let mut interpreter = Interpreter::default();
    for expr in expressions {
        expr.map(|e|interpreter.process(e));
    }

    // let s = "({)-313456758765{34}";
    // let s = "{ 43 life def } def_life def";
    // let tokens:Vec<_> = tokenizer::Tokens::new(s.chars()).collect();
    // println!("Hello, world! Here are the tokens of {s}:\n{:?}\n",tokens);
    // let tokens:Vec<_> = tokenizer::Tokens::new(s.chars()).collect();
    // println!("Hello, world! Here are the tokens of {s}:\n{:?}\n",&tokens);
    // let exprs:Parser<<Vec<Token> as IntoIterator>::IntoIter> = Parser::new(tokens.into_iter());
    // println!("Hello, world! Here are the expressions of {s}:\n{:?}\n",exprs.flatten().collect::<Vec<_>>());

}
