use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Run,
    List,

    #[command(name = "build-code")]
    BuildCode,
}
