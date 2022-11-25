use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Generate {
        #[arg(value_name = "schemas")]
        schemas: Option<Vec<String>>,

        #[arg(long, required = false, value_name = "release")]
        release: Option<String>,
        
        #[arg(long, required = false, default_value = "../extraction")]
        extract_dir: String,
    }
}
