mod args;
mod token_scanner;
mod token;
mod try_parse_from_iter;

fn main(){
    let args = {
        use clap::Parser;
        args::ToastArgs::parse()
    };
    dbg!(args);
}