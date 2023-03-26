#![feature(trace_macros)]
use std::{io::Write};

use crate::{stringy::PoppableString, token_scanner::TokenScanner};

mod args;
mod or;
mod parse;
mod parser;
mod stringy;
mod token;
mod token_scanner;
mod gen;
mod try_parse_from_iter;

fn main() -> std::io::Result<()> {
    let args = {
        use clap::Parser;
        args::ToastArgs::parse()
    };
    // dbg!(&args);
    match &args.mode {
        args::Mode::Compile(_) => {
            // let mut file = File::open(&args.compiler_arguments.source)?;
            // let mut contents = String::new();
            // file.read_to_string(&mut contents)?;
            panic!("Toast cannot yet be run as a compiler")
        }
        args::Mode::Interpret(int) => {
            // let mut buffer = String::new();
            let mut lines = std::io::stdin()
                .lines()
                .map(|a| a.expect("Error reading from the terminal"));
            let prompts = std::iter::from_fn(|| {
                let mut io = std::io::stdout().lock();
                write!(io, "{} ", int.prompt)
                    .and_then(|_| io.flush())
                    .expect("Error writing the prompt string ");
                lines.next()
            });
            for token in TokenScanner::new(prompts) {
                println!("{:?}", token);
            }
        }
    }
    Ok(())
}
