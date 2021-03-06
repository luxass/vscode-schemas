use flate2::read::GzDecoder;
use log::{debug, error, info};
use markdown_gen::markdown::{AsMarkdown, Markdown};
use octocrab::models::repos::{Object, Ref};
use octocrab::params::repos::Reference;
use octocrab::Octocrab;
use regex::Regex;
use schema_lib::{read_schema_list, scan_for_ts_files, write_schema_list, SchemaList};
use std::borrow::Borrow;
use std::fs::File;
use std::io::Cursor;
use std::path::Component::Normal;
use std::path::Path;
use std::process::Command;
use std::{env, fs, io};
use tar::Archive;

// use markdown_gen::markdown::{AsMarkdown, Markdown};
// use octocrab::Octocrab;
// use pulldown_cmark::{html, Options, Parser};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::builder()
        .filter_module("schema_lib", log::LevelFilter::Trace)
        .filter_module("vscode_schemas", log::LevelFilter::Trace)
        .write_style(env_logger::WriteStyle::Always)
        .init();

    info!("Reading schema list");
    let mut schema_list: SchemaList = read_schema_list();

    info!(
        "schema_list -> last_release: {:?}",
        schema_list.last_release
    );

    let extraction_dir: &Path = Path::new("../extraction");

    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");

    let octocrab = Octocrab::builder().personal_token(github_token).build()?;

    let repo = octocrab.repos("microsoft", "vscode");

    let last_release = repo.releases().get_latest().await?;
    let last_release_tag_name = last_release.tag_name;

    let last_release_tag_name_v = format!("v{}", last_release_tag_name.clone());
    if schema_list.last_release == last_release_tag_name {
        info!("no new releases");
        return Ok(());
    }

    info!("latest release: {}", last_release_tag_name);

    let tag: Reference = Reference::Tag(last_release_tag_name.clone());

    let _ref: Ref = repo.get_ref(&tag).await?;

    let long_sha = if let Object::Commit { sha, url: _ } = _ref.object {
        sha
    } else {
        panic!("invalid ref");
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

    let ts_files = scan_for_ts_files(src_folder.join("src").to_str().unwrap())?;
    info!("found {} typescript files.", ts_files.len());

    let mut schema_paths = Vec::<String>::new();

    let re = Regex::new(r"vscode://schemas(?:/\w+)+").unwrap();
    for item in ts_files {
        let contents = fs::read_to_string(item).expect("failed to read file");

        let lines = contents.lines();

        lines.for_each(|line| {
            let captures = re.captures(line);
            if let Some(captures) = captures {
                let path = captures.get(0).map_or("", |m| m.as_str()).to_string();
                debug!("{:?}", path);
                schema_paths.push(path);
            }
        });
    }

    // To ensure that all items in schema_paths are valid, vscode-schemas
    schema_paths = schema_paths
        .iter()
        .map(|s| s.clone())
        .filter(|v| v.contains("vscode://schemas"))
        .collect();

    schema_paths.sort();
    info!("SCHEMAS = {:?}", schema_paths);

    schema_list = SchemaList {
        last_release: last_release_tag_name,
        schemas: schema_paths,
    };
    // write_schema_list(schema_list);

    // if Path::new(src_folder.to_str().unwrap()).exists() {
    //     fs::remove_dir_all(src_folder.to_str().unwrap()).unwrap();
    // }
    //
    // let tar_gz_file_path = extraction_dir.join(&tar_gz_file);
    //
    // if Path::new(&tar_gz_file_path).exists() {
    //     fs::remove_file(&tar_gz_file_path).unwrap();
    // }

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .current_dir("./")
            .args(["/C", "dir"])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .current_dir(src_folder.clone())
            .arg("-c")
            .env("VSCODE_SCHEMAS_AUTO_RUN", "true")
            .env("VSCODE_SCHEMA_OUTPUT_PATH", "../../schema-extraction-output")
            .env(
                "VSCODE_SCHEMA_OVERWRITE_SCHEMA_LIST",
                "../../schema-list.json",
            )
            .arg("yarn install && ls && yarn run compile && ./scripts/code.sh --install-extension ../schema-extractor-0.0.1.vsix")
            // .arg("ls")
            // .arg("yarn run compile")
            // .arg("./scripts/code.sh --install-extension schema-extractor-0.0.1.vsix")
            // .args([
            //     "yarn install",
            //     "ls",
            //     "yarn run compile",
            //     "./scripts/code.sh --install-extension schema-extractor-0.0.1.vsix",
            // ])
            // .arg("ls")
            .output()
            .expect("failed to execute process")
    };

    // if cfg!(target_os = "windows") {
    //     println!("status: {}", output.status);
    // } else {
    //     let gg = Command::new("sh")
    //         .current_dir(src_folder.clone())
    //         .arg("-c")
    //         .env("VSCODE_SCHEMAS_AUTO_RUN", "true")
    //         .env("VSCODE_SCHEMA_OUTPUT_PATH", "../../schema-extraction-output")
    //         .env(
    //             "VSCODE_SCHEMA_OVERWRITE_SCHEMA_LIST",
    //             "../../schema-list.json",
    //         )
    //         .arg("./scripts/code.sh --install-extension ../schema-extractor-0.0.1.vsix")
    //         .output()
    //         .expect("failed to execute process");
    //
    //     let h = gg.stdout;
    //     let g = String::from_utf8_lossy(h.as_slice());
    //     let v: Vec<&str> = g.split("\n").collect();
    //
    //     for x in v {
    //         println!("{:?}", x)
    //     }
    // }


    if cfg!(target_os = "windows") {
        println!("status: {}", output.status);
    } else {
        Command::new("sh")
            .current_dir(src_folder)
            .arg("-c")

            .arg("ls")
            .spawn()
            .expect("failed to execute process");
    }



    let h = output.stdout;
    let g = String::from_utf8_lossy(h.as_slice());
    let v: Vec<&str> = g.split("\n").collect();

    for x in v {
        println!("{:?}", x)
    }

    //init(long_sha).await.expect("TODO: panic message");

    let schemas_dir = Path::new("../schemas");

    let release_folder = schemas_dir.join(last_release_tag_name_v);

    if release_folder.exists() {
        error!("Release folder already exists");
        return Ok(());
    }

    fs::create_dir(release_folder)?;

    let schemas_folders =
        fs::read_dir(schemas_dir)?.filter(|f| f.as_ref().unwrap().path().is_dir());

    let file = File::create(schemas_dir.join("README.md")).unwrap();
    let mut md = Markdown::new(file);
    let version_regex = Regex::new(r"^v(\d+\.)?(\d+\.)?(\*|\d+)$").unwrap();

    md.write("Visual Studio Code Schemas".heading(1)).unwrap();
    md.write("This is a collection of schemas for Visual Studio Code.".quote())
        .unwrap();

    md.write("Versions".heading(2)).unwrap();

    schemas_folders.for_each(|f| {
        for component in f.unwrap().path().components() {
            if let Normal(os_str) = component {
                if let Some(path) = os_str.to_str() {
                    if version_regex.is_match(path) {
                        md.write(path.link_to(path)).unwrap();
                    }
                }
            }
        }
    });

    Ok(())
}
