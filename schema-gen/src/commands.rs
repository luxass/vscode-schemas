use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Generate {
        #[arg(required = true, value_name = "dir")]
        dir: Option<String>,
        
        #[arg(value_name = "schemas")]
        schemas: Option<Vec<String>>,
    },
    Refetch
}
