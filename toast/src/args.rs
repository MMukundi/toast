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

#[derive(Debug, clap::Parser)]
#[command(author, version, about = "The official Toastlang compiler")]
pub struct ToastArgs {
    /// The path to the file which should be compiled
    #[arg(value_name = "source")]
    pub source: String,
    /// The file path where the compiled program should be stored
    #[arg(short, long, value_name = "output")]
    pub output: Option<String>,
    #[arg(default_value = "silent", short, value_name = "verbosity")]
    pub verbosity: CompilerVerbosity,
    // / The amount of logging the
    // #[clap(flatten)]
    // pub verbosity:CompilerVerbosity,
}

// Below is the experimental implementation of CompilerVerbosity which would
// allow flat parsing (toast -v <file> | toast -s <file>), but this option was sacrificed for simplicity

/*
impl CompilerVerbosity{
    const VERBOSE_NAME:&str= "verbose";
    const SILENT_NAME:&str= "silent";

    #[inline]
    fn to_verbosity_opt((presence,verbosity):(bool,Self))->Option<Self>{
        presence.then_some(verbosity)
    }

    fn matched_verbosities(matches:&clap::ArgMatches)->[Option<Self>;2]{
        let verbose = matches.get_flag(Self::VERBOSE_NAME);
        let silent = matches.get_flag(Self::SILENT_NAME);
        [
            (verbose,Self::Verbose),
            (silent,Self::Silent)
        ].map(Self::to_verbosity_opt)
    }
}
impl clap::FromArgMatches for CompilerVerbosity {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        Ok(Self::matched_verbosities(matches).into_iter().flatten().next().unwrap_or_default())
    }

    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        if let Some(_) = Self::matched_verbosities(matches).into_iter().flatten().next() {
            Err(clap::Error::new(clap::error::ErrorKind::ArgumentConflict))
        }else {
            Ok(())
        }
    }
}

impl clap::Args for CompilerVerbosity {
    fn augment_args(cmd: clap::Command) -> clap::Command {
        cmd.arg(clap::Arg::new(Self::VERBOSE_NAME).num_args(0).long(Self::VERBOSE_NAME).short('v').action(clap::ArgAction::SetTrue))
        .arg(clap::Arg::new(Self::SILENT_NAME).num_args(0).long(Self::SILENT_NAME).short('s').action(clap::ArgAction::SetTrue))
        .group(ArgGroup::new("verbosity").arg(Self::VERBOSE_NAME).arg(Self::SILENT_NAME))
    }

    fn augment_args_for_update(cmd: clap::Command) -> clap::Command {
        cmd
    }
}
*/