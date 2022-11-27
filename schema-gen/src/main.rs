use std::{env, error::Error, fs, io, path::Path, time::Duration};

use anyhow::Result;
use clap::Parser;
use flate2::read::GzDecoder;
use log::{debug, error, info, trace, warn};
use octocrab::{
    models::repos::{Object, Ref},
    params::repos::Reference,
    Octocrab,
};

use regex::Regex;
use schema_lib::{
    commands, docker::Docker, is_ci, read_metadata, run_driver, scanner::Scanner, write_metadata,
    Metadata,
};
use semver::{Version, VersionReq};
use std::fs::File;
use std::io::Cursor;
use tar::Archive;
use thirtyfour::{
    prelude::{ElementQueryable, ElementWaitable},
    By, ChromeCapabilities, Key, WebDriver,
};
use tokio::time;
#[derive(Debug, Parser)]
#[command(name = "vsschema")]
#[command(about = "Generate Visual Studio Code Schemas")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: commands::Commands,

    #[arg(
        long,
        required = false,
        global = true,
        default_value = "https://raw.githubusercontent.com/luxass/vscode-schemas/main/metadata.json"
    )]
    metadata_url: String,

    #[arg(long, required = false, global = true)]
    verbose: bool,

    #[arg(long, required = false, global = true, default_value = "../extraction")]
    extract_dir: String,

    #[arg(long, required = false, global = true, value_name = "release")]
    release: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let level_filter = if cli.verbose {
        log::LevelFilter::Trace
    } else {
        log::LevelFilter::Info
    };

    env_logger::builder()
        .filter_module("schema_lib", level_filter)
        .filter_module("schema_gen", level_filter)
        .write_style(env_logger::WriteStyle::Always)
        .init();

    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN env not set");

    let github = Octocrab::builder().personal_token(github_token).build()?;

    let repo = github.repos("microsoft", "vscode");

    match cli.command {
        commands::Commands::Generate {
            schemas: user_schemas,
            dir,
        } => {
            let is_github_actions = is_ci();

            if dir.is_none() {
                error!("dir is required, to generate schemas.");
                return Ok(());
            }

            let metadata = read_metadata(cli.metadata_url).await?;
            let mut schemas = if user_schemas.is_none() {
                metadata.schemas
            } else {
                user_schemas.unwrap()
            };

            let schema_urls = metadata.schema_urls;

            // Filter out schemas
            schemas.retain(|schema| schema.starts_with("vscode://schemas/"));

            if schemas.len() == 0 && schema_urls.len() == 0 {
                warn!("No schemas to generate");
                return Ok(());
            }

            let release = if cli.release.is_none() {
                repo.releases().get_latest().await?
            } else {
                repo.releases().get_by_tag(&cli.release.unwrap()).await?
            };

            if metadata.version == release.tag_name && is_github_actions {
                info!("no new releases");
                return Ok(());
            }

            info!("Generating schemas");
            debug!("Schemas: {:?}", schemas);
            debug!("Schemas Urls: {:?}", schema_urls);

            let container_name = "vscode-schema-server";

            let docker = Docker::new()?;

            docker.build_image().await.expect("failed to build image");

            // Create container
            let create_response = docker
                .create_container(container_name, dir.unwrap())
                .await
                .expect("failed to create container");

            // Start the container
            docker
                .start_container(&create_response.id)
                .await
                .expect("failed to start container");

            let mut chrome_driver = run_driver();
            info!("Chrome driver started");
            info!("id: {}", chrome_driver.id());

            // Just to be sure that the driver and server is ready.
            time::sleep(std::time::Duration::from_secs(20)).await;
            let mut caps = ChromeCapabilities::new();
            if is_github_actions {
                caps.set_headless()?;
            }
            let driver = WebDriver::new("http://localhost:9515", caps).await?;

            driver
                .goto("http://localhost:8000/?folder=/root/vscode-schemas")
                .await?;

            let body = driver.find(By::Tag("body")).await?;

            let workspace_trust = body.query(By::Css("div > div.monaco-dialog-modal-block.dimmed > div > div > div.dialog-buttons-row > div > a:nth-child(1)"))
            .single()
            .await?;

            workspace_trust.wait_until().displayed().await?;
            workspace_trust.click().await?;

            time::sleep(Duration::from_secs(1)).await;
            body.send_keys(String::from(Key::Control + Key::Shift + Key::Alt + "s"))
                .await?;
            debug!("triggered extract schemas");

            time::sleep(Duration::from_secs(10)).await;
            driver.quit().await?;
            chrome_driver
                .kill()
                .expect("chromedriver server process not killed, do manually");

            docker
                .kill(container_name)
                .await
                .expect("failed to kill container");
            time::sleep(Duration::from_secs(5)).await;
            docker
                .destroy(container_name)
                .await
                .expect("failed to remove container");
        }
        commands::Commands::Refetch => {
            let release = if cli.release.is_none() {
                repo.releases().get_latest().await?
            } else {
                // Release 1.39.0 is the first "correct" release from github.
                // But they are going directly from release 1.39.2 to 1.45.0
                // so we are just gonna accept everything over 1.45.0
                let version = VersionReq::parse(">= 1.45.0")?;
                let release_version = cli.release.unwrap();
                if version.matches(&Version::parse(&release_version)?) {
                    debug!("release version: {}", release_version);
                    match repo.releases().get_by_tag(&release_version).await {
                        Ok(release) => release,
                        Err(_) => {
                            error!("Release {} not found", release_version);
                            return Ok(());
                        }
                    }
                } else {
                    error!("Release {} is not supported", &release_version);
                    return Ok(());
                }
            };

            let tag: Reference = Reference::Tag(release.tag_name.clone());
            trace!("Tag::Reference: {:?}", tag);

            let _ref: Ref = repo.get_ref(&tag).await?;
            trace!("Ref: {:?}", _ref);

            let long_sha = if let Object::Tag { sha, url: _ } = _ref.object {
                sha
            } else if let Object::Commit { sha, url: _ } = _ref.object {
                sha
            } else {
                panic!("What is this?");
            };

            let short_sha = &long_sha[0..7];

            debug!("sha for last release: {}", long_sha);

            let unpack_name = format!("microsoft-vscode-{}", short_sha);
            debug!("unpack_name: {}", unpack_name);

            let extraction_dir: &Path = Path::new(&cli.extract_dir);
            if !extraction_dir.exists() {
                info!("Creating extraction directory");
                std::fs::create_dir(extraction_dir)?;
            }

            let src_folder = extraction_dir.join(unpack_name);

            // microsoft-vscode-1.67.2-0-gc3511e6.tar.gz
            let tar_gz_file = format!(
                "microsoft-vscode-{release}-0-{short_sha}.tar.gz",
                release = release.tag_name,
                short_sha = short_sha
            );

            if !Path::new(src_folder.to_str().unwrap()).exists() {
                let res = repo.download_tarball(tag).await?;

                let mut file = File::create(extraction_dir.join(&tar_gz_file))?;

                let bytes = res.bytes().await.expect("failed to read bytes");

                let mut content = Cursor::new(bytes);

                io::copy(&mut content, &mut file).expect("failed to write file");

                let tar_gz = File::open(extraction_dir.join(&tar_gz_file))?;

                let tar = GzDecoder::new(tar_gz);
                let mut archive = Archive::new(tar);
                archive.unpack(extraction_dir.join("."))?;
            }

            let mut scanner = Scanner::new(short_sha.to_string(), release.tag_name.clone());
            info!("Getting files for scanning");
            scanner.scan_files(src_folder.to_str().unwrap())?;

            info!("Scanning files");
            scanner.parse_files()?;

            let metadata = Metadata {
                version: release.tag_name,
                schemas: scanner.schemas,
                schema_urls: scanner.schema_urls,
            };

            debug!("Schemas: {:?}", metadata.schemas);
            debug!("Schemas Urls: {:?}", metadata.schema_urls);

            if Path::new(src_folder.to_str().unwrap()).exists() {
                fs::remove_dir_all(src_folder.to_str().unwrap()).unwrap();
            }

            let tar_gz_file_path = extraction_dir.join(&tar_gz_file);

            if Path::new(&tar_gz_file_path).exists() {
                fs::remove_file(&tar_gz_file_path).unwrap();
            }

            let url_re = Regex::new(r"https?")?;

            if url_re.is_match(&cli.metadata_url) {
                warn!("The option --metadata-url, was not a file located on the filesystem");
                warn!("We cant write the metadata.");

                return Ok(());
            }

            info!("Metadata written to {}", &cli.metadata_url);
            write_metadata(metadata, cli.metadata_url)?;
        }
    }

    Ok(())
}
