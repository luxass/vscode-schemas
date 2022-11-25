use std::{env, error::Error, fs, io, path::Path, time::Duration};

use anyhow::Result;
use clap::Parser;
use flate2::read::GzDecoder;
use log::{debug, info, warn};
use octocrab::{
    models::repos::{Object, Ref},
    params::repos::Reference,
    Octocrab,
};
use regex::Regex;
use schema_lib::{
    commands, docker::Ducker, is_ci, read_metadata, run_driver, scan_for_files, set_default_env,
    write_metadata, Metadata,
};
use std::fs::File;
use std::io::Cursor;
use tar::Archive;
use thirtyfour::{
    prelude::{ElementQueryable, ElementWaitable},
    By, ChromeCapabilities, Key, WebDriver,
};
use tokio::time;
use url::Url;
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
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let is_github_actions = is_ci();
    let level_filter = if is_github_actions {
        log::LevelFilter::Info
    } else {
        log::LevelFilter::Trace
    };

    env_logger::builder()
        .filter_module("schema_lib", level_filter)
        .filter_module("schema_gen", level_filter)
        .write_style(env_logger::WriteStyle::Always)
        .init();

    let cli = Cli::parse();

    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");

    let github = Octocrab::builder().personal_token(github_token).build()?;

    let repo = github.repos("microsoft", "vscode");

    match cli.command {
        commands::Commands::Generate {
            schemas: user_schemas,
            release: user_release,
            extract_dir,
        } => {
            let mut metadata = read_metadata(cli.metadata_url).await?;
            let mut schemas = if user_schemas.is_none() {
                metadata.schemas
            } else {
                user_schemas.unwrap()
            };

            // Filter out schemas
            schemas.retain(|schema| schema.starts_with("vscode://schemas/"));

            if schemas.len() == 0 {
                warn!("No schemas to generate");
                return Ok(());
            }

            let release = if user_release.is_none() {
                repo.releases().get_latest().await?
            } else {
                repo.releases().get_by_tag(&user_release.unwrap()).await?
            };

            if metadata.version == release.tag_name {
                info!("no new releases");
                return Ok(());
            }

            info!("Generating schemas");
            debug!("Schemas: {:?}", schemas);

            info!("latest release: {}", release.tag_name);
            let tag: Reference = Reference::Tag(release.tag_name.clone());

            let _ref: Ref = repo.get_ref(&tag).await?;

            let long_sha = if let Object::Commit { sha, url: _ } = _ref.object {
                sha
            } else {
                panic!("invalid ref");
            };

            let short_sha = &long_sha[0..7];

            debug!("sha for last release: {}", long_sha);

            let unpack_name = format!("microsoft-vscode-{}", short_sha);
            info!("unpack_name: {}", unpack_name);

            let extraction_dir: &Path = Path::new(extract_dir.as_str());
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

            let files = scan_for_files(src_folder.to_str().unwrap())?;
            info!("found {} files.", files.len());

            let mut schema_paths = Vec::<String>::new();

            let re = Regex::new(r"vscode://schemas(?:/\w+)+").unwrap();
            for file in files {
                let contents = fs::read_to_string(&file).expect("failed to read file");

                let lines = contents.lines();

                lines.for_each(|line| {
                    let captures = re.captures(line);
                    if let Some(captures) = captures {
                        let path = captures.get(0).map_or("", |m| m.as_str()).to_string();
                        debug!("{:?} - {:?}", path, &file);
                        schema_paths.push(path);
                    }
                });
            }

            schema_paths = schema_paths
                .iter()
                .map(|x| x.to_string())
                .collect::<std::collections::HashSet<_>>()
                .iter()
                .map(|x| x.to_string())
                .collect();

            metadata = Metadata {
                version: release.tag_name,
                schemas: schema_paths,
            };

            // write_metadata(metadata, metadata_path)?;

            if Path::new(src_folder.to_str().unwrap()).exists() {
                fs::remove_dir_all(src_folder.to_str().unwrap()).unwrap();
            }

            let tar_gz_file_path = extraction_dir.join(&tar_gz_file);

            if Path::new(&tar_gz_file_path).exists() {
                fs::remove_file(&tar_gz_file_path).unwrap();
            }

            // let container_name = "vscode-schema-server";
            // let docker = Ducker::new()?;

            // docker
            //     .build_image(metadata_path)
            //     .await
            //     .expect("failed to build image");

            // // Create container
            // let create_response = docker
            //     .create_container(container_name)
            //     .await
            //     .expect("failed to create container");

            // // Start the container
            // docker
            //     .start_container(&create_response.id)
            //     .await
            //     .expect("failed to start container");

            // let mut chrome_driver = run_driver();
            // info!("Chrome driver started");
            // info!("id: {}", chrome_driver.id());

            // // Just to be sure that the driver and server is ready.
            // time::sleep(std::time::Duration::from_secs(20)).await;
            // let mut caps = ChromeCapabilities::new();
            // if github_actions == "true" {
            //     caps.set_headless()?;
            // }
            // let driver = WebDriver::new("http://localhost:9515", caps).await?;

            // driver
            //     .goto("http://localhost:8000/?folder=/root/vscode-schemas")
            //     .await?;

            // let body = driver.find(By::Tag("body")).await?;

            // let workspace_trust = body.query(By::Css("div > div.monaco-dialog-modal-block.dimmed > div > div > div.dialog-buttons-row > div > a:nth-child(1)"))
            // .single()
            // .await?;

            // workspace_trust.wait_until().displayed().await?;
            // workspace_trust.click().await?;

            // time::sleep(Duration::from_secs(1)).await;
            // body.send_keys(String::from(Key::Control + Key::Shift + Key::Alt + "s"))
            //     .await?;
            // debug!("triggered extract schemas");

            // time::sleep(Duration::from_secs(10)).await;
            // driver.quit().await?;
            // chrome_driver
            //     .kill()
            //     .expect("chromedriver server process not killed, do manually");

            // docker
            //     .kill(container_name)
            //     .await
            //     .expect("failed to kill container");
            // time::sleep(Duration::from_secs(5)).await;
            // docker
            //     .destroy(container_name)
            //     .await
            //     .expect("failed to remove container");
        }
    }

    Ok(())
}
