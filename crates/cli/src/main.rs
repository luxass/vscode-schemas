use anyhow::Result;
use clap::{arg, command, Parser};
use log::{info, LevelFilter};

mod commands;

#[derive(Parser)]
#[command(name = "vsschema")]
#[command(bin_name = "vsschema")]
#[command(about = "Generate Visual Studio Code Schemas")]
#[command(version = "0.1.0")]
struct Args {
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
async fn main() -> Result<()> {
    let args = Args::parse();

    env_logger::builder()
        .filter_module("cli", args.log)
        .filter_module("schema_core", args.log)
        .filter_module("code_agent", args.log)
        .write_style(env_logger::WriteStyle::Always)
        .init();

    let release = schema_core::github::parse_release(args.release).await?;

    match args.command {
        commands::Commands::Run => {
            schema_core::agent::run_code_agent(&release).await?;
        }
        commands::Commands::List { show } => match show {
            commands::Show::Releases => {
                schema_core::github::list_releases().await?;
            }
            commands::Show::Schemas => {
                schema_core::github::list_schemas().await?;
            }
        },
        commands::Commands::BuildCode => {
            schema_core::agent::build_code_agent(&release)
                .await?;
        }
        commands::Commands::Generate => {
            schema_core::scanner::generate_schemas(&release)
                .await?;
        }
    }

    if args.cleanup {
        schema_core::agent::cleanup(&release).await?;
        info!("Cleanup complete");
    }

    Ok(())
}
