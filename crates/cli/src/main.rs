use clap::{arg, command, Parser};
use log::{info, LevelFilter};
mod commands;

#[derive(Parser)]
#[command(name = "vsschema")]
#[command(bin_name = "vsschema")]
#[command(about = "Generate Visual Studio Code Schemas")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: commands::Commands,

    #[arg(long, required = false, global = true)]
    release: Option<String>,

    #[arg(
        long,
        required = false,
        global = true,
        value_enum,
        default_value = "info"
    )]
    log: LevelFilter,

    #[arg(long, required = false, global = true)]
    cleanup: bool,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    env_logger::builder()
        .filter_module("cli", cli.log)
        .filter_module("schema_core", cli.log)
        .filter_module("code_agent", cli.log)
        .write_style(env_logger::WriteStyle::Always)
        .init();

    let release = schema_core::github::parse_release(cli.release)
        .await
        .unwrap();

    match cli.command {
        commands::Commands::Run => {
            schema_core::agent::run_code_agent(&release)
                .await
                .unwrap();
        }
        commands::Commands::List { show } => match show {
            commands::Show::Releases => {
                schema_core::github::list_releases().await.unwrap();
            }
            commands::Show::Schemas => {
                schema_core::github::list_schemas().await.unwrap();
            }
        },
        commands::Commands::BuildCode => {
            schema_core::agent::build_code_agent(&release)
                .await
                .unwrap();
        }
        commands::Commands::Generate => {
            schema_core::scanner::generate_schemas(&release)
                .await
                .unwrap();
        }
    }

    if cli.cleanup {
        schema_core::agent::cleanup(&release).await.unwrap();
        info!("Cleanup complete");
    }
}
