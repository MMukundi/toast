//! This module contains all processing for the command line arguments
//! To start, take a look at [`ToastArgs`]'

use clap::{Args, FromArgMatches};

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
    source: String,
    /// The file path where the compiled program should be stored
    #[arg(short, long, value_name = "output")]
    output: Option<String>,
}
#[derive(Debug, Clone, clap::Args)]
#[command(about = "Run the toast interpreter")]
pub struct InterpreterArguments {}
#[derive(Debug, Clone,clap::Subcommand)]
pub enum Mode {
    Compiler(CompilerArguments),
    Interpreter(InterpreterArguments),
}
// impl FromArgMatches for Mode {
//     fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
//         CompilerArguments::from_arg_matches(matches)
//             .map(Mode::Compiler)
//             .or_else(|_| InterpreterArguments::from_arg_matches(matches).map(Mode::Interpreter))
//     }
//     fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
//         match self {
//             Mode::Compiler(c) => c.update_from_arg_matches(matches),
//             Mode::Interpreter(i) => i.update_from_arg_matches(matches),
//         }
//     }
// }
impl Args for Mode {
    fn augment_args(cmd: clap::Command) -> clap::Command {
        let interpret = InterpreterArguments::augment_args(clap::Command::new("interpret"));
        let compile = CompilerArguments::augment_args(clap::Command::new("compile"));
        cmd.subcommand(compile).subcommand(interpret)
    }

    fn augment_args_for_update(cmd: clap::Command) -> clap::Command {
        Self::augment_args(cmd)
    }
}

#[derive(Debug, clap::Parser)]
#[command(author, version, about = "The official Toastlang compiler")]
pub struct ToastArgs {
    /// The arguments if `toast` is run as a compiler
    #[clap(flatten)]
    // #[arg(default_value = "--compiler")]
    pub mode: Mode,

    #[arg(default_value = "silent", short, value_name = "verbosity")]
    pub verbosity: CompilerVerbosity,
    // / The amount of logging the
    // #[clap(flatten)]
    // pub verbosity:CompilerVerbosity,
}
