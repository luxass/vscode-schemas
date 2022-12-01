use log::{error, info};
use octocrab::models::repos::Release;
use semver::{VersionReq, Version};

use crate::errors::{Error, self};

pub async fn get_release_by_tag(release: String) -> Result<String, Error> {
    let release = reqwest::Client::new()
        .get(format!(
            "https://api.github.com/repos/microsoft/vscode/releases/tags/{tag}",
            tag = release
        ))
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "https://github.com/luxass/vscode-schemas")
        .send()
        .await?
        .json::<Release>()
        .await?;

    Ok(release.tag_name)
}

pub async fn latest_release() -> Result<String, Error> {
    let release = reqwest::Client::new()
        .get("https://api.github.com/repos/microsoft/vscode/releases/latest")
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "https://github.com/luxass/vscode-schemas")
        .send()
        .await?
        .json::<Release>()
        .await?;

    Ok(release.tag_name)
}

pub async fn parse_release(release_arg: Option<String>) -> Result<String, Error> {
    let release = if release_arg.is_none() {
        info!("No release specified, using latest");
        latest_release().await?
    } else {
        let version = VersionReq::parse(">= 1.45.0")?;
        let release_version = release_arg.unwrap();
        info!("Using release {}", &release_version);
        if version.matches(&Version::parse(&release_version)?) {
            match get_release_by_tag(release_version.clone()).await {
                Ok(release) => release,
                Err(e) => {
                    error!("Error getting release: {:?}", e);
                    error!("Release {} not found", &release_version);
                    return Err(errors::Error::ReleaseNotFound);
                }
            }
        } else {
            error!("Release {} is not supported", &release_version);
            return Err(errors::Error::ReleaseNotSupported)
        }
    };
    Ok(release)
}

pub async fn list_schemas() -> Result<(), Error> {
    Ok(())
}
