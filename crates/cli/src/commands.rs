use std::fmt::Debug;

use clap::{Subcommand, ValueEnum};

#[derive(Subcommand)]
pub enum Commands {
    Run,
    List {
        #[arg(long, value_enum, default_value_t = Show::Releases)]
        show: Show,
    },
    #[command(name = "build-code")]
    BuildCode,
    Generate,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Show {
    Releases,
    Schemas,
}
