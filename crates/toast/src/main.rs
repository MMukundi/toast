mod tokenizer;
mod tokens;
mod util;

fn main() {
    // let s = "({)-313456758765{34}";
    // let tokens:Vec<_> = tokenizer::Tokens::new(s.chars()).collect();
    // println!("Hello, world! Here are the tokens of {s}:\n{:?}\n",tokens);
    let s = "{ 0b11.1 0xFF.F 0o77.7 } car def";
    let tokens:Vec<_> = tokenizer::Tokens::new(s.chars()).collect();
    println!("Hello, world! Here are the tokens of {s}:\n{:?}\n",tokens);
}
