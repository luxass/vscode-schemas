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
    Build,
    #[command(name = "build-agent")]
    BuildAgent {
        #[arg(long, required = false)]
        release: Option<String>,
    }
}
