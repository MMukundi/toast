mod args;

fn main(){
    let args = {
        use clap::Parser;
        args::ToastArgs::parse()
    };
    dbg!(args);
}