use anyhow::Result;
use bollard::{
    container::{Config, CreateContainerOptions, StartContainerOptions},
    image::RemoveImageOptions,
    Docker,
};
use log::info;

use crate::{errors::Error, image};

pub async fn build_code_agent(release: String) -> Result<(), Error> {
    info!("Building Code Agent for release {}", release);
    let docker = Docker::connect_with_socket_defaults()?;
    let exists = image::check_for_image(&docker, format!("vscode-build-agent:{}", release)).await?;
    if exists {
        info!("Image already exists, skipping build");
        return Ok(());
    } else {
        image::build_code_image(&docker, release).await?;

        info!("Finished building Code Agent");
    }

    Ok(())
}

pub async fn run_code_agent(release: String) -> Result<(), Error> {
    info!("Running Code Agent for release {}", release);
    let docker = Docker::connect_with_socket_defaults()?;

    let tag = format!("vscode-build-agent:{}", release);

    let exists = image::check_for_image(&docker, tag.clone()).await?;
    if !exists {
        info!("Image does not exist, building");
        image::build_code_image(&docker, release.clone()).await?;
    }

    docker
        .create_container(
            Some(CreateContainerOptions {
                name: String::from("vscode-schema-server"),
                ..Default::default()
            }),
            Config {
                image: Some(tag),
                // hostname: Some("vscode".to_string()),
                // host_config: Some(HostConfig {
                //     network_mode: Some("host".to_string()),
                //     port_bindings: Some(port_map),
                //     // binds: Some(vec![volume]),
                //     mounts: Some(vec![Mount {
                //         target: Some(String::from("/root/vscode-schemas")),
                //         source: Some(dir),
                //         typ: Some(MountTypeEnum::BIND),
                //         consistency: Some(String::from("default")),
                //         ..Default::default()
                //     }]),
                //     ..Default::default()
                // }),
                ..Default::default()
            },
        )
        .await?;
    
    docker.start_container("vscode-schema-server", None::<StartContainerOptions<String>>).await?;

    Ok(())
}

pub async fn cleanup(release: String) -> Result<(), Error> {
    let docker = Docker::connect_with_socket_defaults()?;

    let image_name = format!("vscode-build-agent:{}", release);

    docker
        .remove_image(
            image_name.as_str(),
            Some(RemoveImageOptions {
                force: false,
                noprune: true,
            }),
            None,
        )
        .await?;

    Ok(())
}
