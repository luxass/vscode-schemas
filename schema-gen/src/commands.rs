use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Generate {
        #[arg(value_name = "schemas")]
        schemas: Option<Vec<String>>,
    },
    Refetch
}
