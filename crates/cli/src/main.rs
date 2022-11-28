use clap::{command, Parser};
use log::info;
mod commands;

#[derive(Debug, Parser)]
#[command(name = "vsschema")]
#[command(bin_name = "vsschema")]
#[command(about = "Generate Visual Studio Code Schemas")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: commands::Commands,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        commands::Commands::Run {} => {
            info!("Run Command");
        }
        commands::Commands::List => {
            info!("List Command");
            // schema_core::list_schemas();
        }
        commands::Commands::Dev { command } => match command {
            commands::DevCommands::Build => {
                info!("Dev Build Command");

                code_builder::build_code().unwrap();
                
            }
        },
    }
}
