use std::{fs::File, io::Read};

use crate::token_scanner::TokenScanner;

mod args;
mod parser;
mod token;
mod token_scanner;
mod try_parse_from_iter;

fn main() -> std::io::Result<()> {
    let args = {
        use clap::Parser;
        args::ToastArgs::parse()
    };
    dbg!(&args);
    // let mut file = File::open(&args.compiler_arguments.source)?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;
    // if args.interpreter_mode {
    //     for token in TokenScanner::new(contents.lines().map(|l|l.to_string())){
    //         dbg!(token);
    //     }
    // }else{
    //     panic!("Toast cannot yet be run as a compiler")
    // }
    Ok(())
}
