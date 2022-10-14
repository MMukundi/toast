extern crate core;

use std::io;
use std::io::{BufRead, Read, Write};
use codegen::interpreter::{Prompt, Prompter};

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

pub struct ToastRead<'s>(std::io::StdinLock<'s>);
impl Read for ToastRead<'_>{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.0.read(buf)
    }
}
impl BufRead for ToastRead<'_>{
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.0.fill_buf()
    }

    fn consume(&mut self, amt: usize) {
        self.0.consume(amt)
    }
}
pub struct ToastWrite<'s>(std::io::StdoutLock<'s>);
impl Write for ToastWrite<'_>{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.0.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.0.flush()
    }
}
impl Prompt for ToastWrite<'_>{
    fn write_prompt(&mut self) {
        self.0.write(b">> ").expect("Error_prompting");
    }

    fn write_continue_prompt(&mut self) {
        self.0.write(b"-  ").expect("Error_con_prompting");
    }
}

fn main() {
    // let mut stdin = std::io::stdin().lock();
    Interpreter::run_in_prompter(
        Prompter::new(
            ToastRead(std::io::stdin().lock()), 
            ToastWrite(std::io::stdout().lock()), 
        )
    );

    // let s = "({)-313456758765{34}";
    // let s = "{ 43 life def } def_life def";
    // let tokens:Vec<_> = tokenizer::Tokens::new(s.chars()).collect();
    // println!("Hello, world! Here are the tokens of {s}:\n{:?}\n",tokens);
    // let tokens:Vec<_> = tokenizer::Tokens::new(s.chars()).collect();
    // println!("Hello, world! Here are the tokens of {s}:\n{:?}\n",&tokens);
    // let exprs:Parser<<Vec<Token> as IntoIterator>::IntoIter> = Parser::new(tokens.into_iter());
    // println!("Hello, world! Here are the expressions of {s}:\n{:?}\n",exprs.flatten().collect::<Vec<_>>());

}
