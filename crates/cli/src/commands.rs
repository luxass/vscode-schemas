use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Run {},
    List,
    Dev {
        #[clap(subcommand)]
        command: DevCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum DevCommands {
    Build
}
