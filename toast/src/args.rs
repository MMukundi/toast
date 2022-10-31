//! This module contains all processing for the command line arguments
//! To start, take a look at [`ToastArgs`]'

pub const TOAST_ASCII: &str = r#"================
 ###### ######
#......#......#
#.............#
 #...........#
 #...........#
 #...........#
  ###########
================"#;

#[derive(Debug, Clone, Default, clap::ValueEnum)]
pub enum CompilerVerbosity {
    Verbose,
    #[default]
    Silent,
}
#[derive(Debug, Clone, clap::Args)]
#[command(about = "Run the toast compiler")]
pub struct CompilerArguments {
    /// The path to the file which should be compiled
    // #[arg(value_name = "source")]
    pub source: String,
    /// The file path where the compiled program should be stored
    #[arg(short, long, value_name = "output")]
    pub output: Option<String>,
}
#[derive(Debug, Clone, clap::Args)]
#[command(about = "Run the toast interpreter")]
pub struct InterpreterArguments {
    #[arg(short, long, default_value = ">>", value_name = "prompt")]
    pub prompt: String,
}
#[derive(Debug, Clone, clap::Parser)]
pub enum Mode {
    Compile(CompilerArguments),
    Interpret(InterpreterArguments),
}

#[derive(Debug, clap::Parser)]
#[command(author, version, about = "The official Toastlang compiler")]
pub struct ToastArgs {
    #[command(subcommand)]
    pub mode: Mode,

    #[arg(default_value = "silent", short, value_name = "verbosity")]
    pub verbosity: CompilerVerbosity,
}
