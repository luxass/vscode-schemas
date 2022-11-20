use std::{env, error::Error, fs, io, path::Path, time::Duration};

use anyhow::Result;
use flate2::read::GzDecoder;
use log::{debug, info};
use octocrab::{
    models::repos::{Object, Ref},
    params::repos::Reference,
    Octocrab,
};
use regex::Regex;
use scanner_lib::{docker, read_metadata, run_driver, scan_for_files, Metadata};
use std::fs::File;
use std::io::Cursor;
use tar::Archive;
use thirtyfour::{
    prelude::{ElementQueryable, ElementWaitable},
    By, DesiredCapabilities, WebDriver,
};
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::builder()
        .filter_module("scanner_lib", log::LevelFilter::Trace)
        .filter_module("scanner", log::LevelFilter::Trace)
        .write_style(env_logger::WriteStyle::Always)
        .init();

    info!("Reading metadata");
    let mut metadata: Metadata = read_metadata();

    info!("metadata version: {:?}", metadata.version);

    let extraction_dir: &Path = Path::new("../extraction");
    if !extraction_dir.exists() {
        info!("Creating extraction directory");
        std::fs::create_dir(extraction_dir)?;
    }

    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");

    let octocrab = Octocrab::builder().personal_token(github_token).build()?;

    let repo = octocrab.repos("microsoft", "vscode");

    let last_release = repo.releases().get_latest().await?;
    let last_release_tag_name = last_release.tag_name;

    let _last_release_tag_name_v = format!("v{}", last_release_tag_name.clone());
    if metadata.version == last_release_tag_name {
        info!("no new releases");
        return Ok(());
    }

    info!("latest release: {}", last_release_tag_name);
    let tag: Reference = Reference::Tag(last_release_tag_name.clone());

    let _ref: Ref = repo.get_ref(&tag).await?;

    let long_sha = if let Object::Tag { sha, url: _ } = _ref.object {
        sha
    } else {
        panic!("not a commit");
    };

    let short_sha = &long_sha[0..7];

    info!("sha for last release: {}", long_sha);

    let unpack_name = format!("microsoft-vscode-{}", short_sha);
    info!("unpack_name: {}", unpack_name);

    let src_folder = extraction_dir.join(unpack_name);

    // microsoft-vscode-1.67.2-0-gc3511e6.tar.gz
    let tar_gz_file = format!(
        "microsoft-vscode-{release}-0-{short_sha}.tar.gz",
        release = last_release_tag_name,
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

    // TODO: Read extensions folders here.

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

    schema_paths.sort();
    info!("SCHEMAS = {:?}", schema_paths);

    metadata = Metadata {
        version: last_release_tag_name,
        schemas: schema_paths,
    };

    info!("Metadata {:?}", metadata);

    if Path::new(src_folder.to_str().unwrap()).exists() {
        fs::remove_dir_all(src_folder.to_str().unwrap()).unwrap();
    }

    let tar_gz_file_path = extraction_dir.join(&tar_gz_file);

    if Path::new(&tar_gz_file_path).exists() {
        fs::remove_file(&tar_gz_file_path).unwrap();
    }

    docker::init().await.expect("failed to init docker");

    // Just to be sure, Server is started.
    tokio::time::sleep(std::time::Duration::from_secs(20)).await;

    let mut chrome_driver = run_driver();
    info!("Chrome driver started");
    info!("id: {}", chrome_driver.id());
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://localhost:9515", caps).await?;

    driver
        .goto("http://localhost:8000/?folder=/root/vscode-schemas")
        .await?;

    let body = driver.find(By::Tag("body")).await?;

    // let explorer_open = driver.find(By::Id("workbench.view.explorer")).await;

    // if explorer_open.is_err() {
    //     info!("explorer not open, opening it");
    //     time::sleep(Duration::from_secs(2)).await;
    //     let explorer_toggle = driver
    //         .find(By::XPath(
    //             "//*[@id=\"workbench.parts.activitybar\"]/div/div[2]/div/ul/li[1]/a",
    //         ))
    //         .await?;
    //     explorer_toggle.click().await?;
    // }

    // time::sleep(Duration::from_secs(2)).await;

    // body.send_keys(Key::Control + "k o".to_string()).await?;
    // info!("opened file picker");

    // let folder_input = body.query(By::Css("div > div.quick-input-widget.show-file-icons > div.quick-input-header > div.quick-input-and-message > div.quick-input-filter > div.quick-input-box > div > div.monaco-inputbox.idle > div > input"))
    // .single()
    // .await?;

    // folder_input.wait_until().displayed().await?;
    // folder_input.send_keys("vscode-schemas/").await?;
    // folder_input.send_keys("" + Key::Enter).await?;

    let workspace_trust = body.query(By::Css("div > div.monaco-dialog-modal-block.dimmed > div > div > div.dialog-buttons-row > div > a:nth-child(1)"))
    .single()
    .await?;

    workspace_trust.wait_until().displayed().await?;
    workspace_trust.click().await?;

    time::sleep(Duration::from_secs(10)).await;
    driver.quit().await?;
    chrome_driver
        .kill()
        .expect("chromedriver server process not killed, do manually");

    Ok(())
}
