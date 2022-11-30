use clap::{arg, command, Parser, ValueEnum};
use log::{info, LevelFilter};
mod commands;

#[derive(Debug, Parser)]
#[command(name = "vsschema")]
#[command(bin_name = "vsschema")]
#[command(about = "Generate Visual Studio Code Schemas")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: commands::Commands,

    #[arg(
        long,
        required = false,
        global = true,
        value_enum,
        default_value = "info"
    )]
    log: LevelFilter,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    env_logger::builder()
        .filter_module("cli", cli.log)
        .filter_module("schema_core", cli.log)
        .filter_module("code_builder", cli.log)
        .write_style(env_logger::WriteStyle::Always)
        .init();

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

                // code_builder::build_code().unwrap();
            }
            commands::DevCommands::BuildAgent {
                release
            } => {
                info!("Dev Build Agent Command");
                code_builder::build_code_agent(release).await.unwrap();
            }
        },
    }
}
