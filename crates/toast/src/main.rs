extern crate core;

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

fn main() {
    let prompt_str = ">> ";
    print!("{}",prompt_str);
    let mut input = std::io::stdin().lines().inspect(|_|{
        print!("{}",prompt_str);
    }).flatten().flat_map(|c|c.chars().chain(std::iter::once('\n')).collect::<Vec<_>>());

    let tokens = tokenizer::Tokens::new(input);
    let exprs = Parser::new(tokens);
    let mut interpreter = Interpreter::default();

    for expr in exprs {
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
