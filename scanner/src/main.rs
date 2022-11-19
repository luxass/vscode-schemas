use std::{env, io, path::Path};

use anyhow::Result;
use flate2::read::GzDecoder;
use log::info;
use octocrab::{
    models::repos::{Object, Ref},
    params::repos::Reference,
    Octocrab,
};
use scanner_lib::{read_metadata, Metadata};
use std::fs::File;
use std::io::Cursor;
use tar::Archive;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder()
        .filter_module("scanner_lib", log::LevelFilter::Trace)
        .filter_module("scanner", log::LevelFilter::Trace)
        .write_style(env_logger::WriteStyle::Always)
        .init();

    info!("Reading metadata");
    let mut metadata: Metadata = read_metadata();

    info!("metadata version: {:?}", metadata.version);

    let extraction_dir: &Path = Path::new("../extraction");

    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");

    let octocrab = Octocrab::builder().personal_token(github_token).build()?;

    let repo = octocrab.repos("microsoft", "vscode");

    let last_release = repo.releases().get_latest().await?;
    let last_release_tag_name = last_release.tag_name;

    let last_release_tag_name_v = format!("v{}", last_release_tag_name.clone());
    if metadata.version == last_release_tag_name {
        info!("no new releases");
        return Ok(());
    }

    info!("latest release: {}", last_release_tag_name);
    let tag: Reference = Reference::Tag(last_release_tag_name.clone());
    info!("getting tag: {:?}", tag);

    let _ref: Ref = repo.get_ref(&tag).await?;
    info!("ref: {:?}", _ref.object);
    let url = if let Object::Tag { url, .. } = _ref.object.clone() {
        url
    } else {
        panic!("not a tag");
    };
    info!("url: {}", url);

    let long_sha = if let Object::Commit { sha, url: _ } = _ref.object {
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

    Ok(())
}
