use bollard::image::BuildImageOptions;
use bollard::Docker;

use bollard::container::{Config, CreateContainerOptions, LogsOptions, StartContainerOptions};
use bollard::errors::Error;
use bollard::service::{HostConfig, PortBinding, PortMap};
use std::collections::HashMap;
use std::env;
use std::io::Write;

use futures_util::stream::StreamExt;
use futures_util::TryStreamExt;

use crate::set_default_env;

pub fn build_dockerfile() -> String {
    format!(
        r#"FROM ubuntu:22.04

        # hadolint ignore=DL3008
        RUN apt-get update && export DEBIAN_FRONTEND=noninteractive && apt-get install -y --no-install-recommends \
            # vscode requirements
            gnome-keyring wget curl python3-minimal ca-certificates \
            # development tools
            git build-essential \
            # clean up
            && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*
            
        RUN wget -q -O- https://aka.ms/install-vscode-server/setup.sh | sh

        
        ENTRYPOINT [ "/bin/sh", "-c", "code-server serve-local --accept-server-license-terms --disable-telemetry --without-connection-token --host 0.0.0.0 --start-server --install-extension luxass.vscode-schema-extractor" ]
        EXPOSE 8000
    "#,
    )
}

pub async fn init() -> Result<(), Error> {
    let docker = Docker::connect_with_socket_defaults()?;
    let dockerfile = build_dockerfile();
    let mut header = tar::Header::new_gnu();
    header.set_path("Dockerfile")?;
    header.set_size(dockerfile.len() as u64);
    header.set_mode(0o755);
    header.set_cksum();
    let mut tar = tar::Builder::new(Vec::new());
    tar.append(&header, dockerfile.as_bytes())?;

    let uncompressed = tar.into_inner()?;
    let mut c = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    c.write_all(&uncompressed)?;
    let compressed = c.finish()?;

    let build_image_options = BuildImageOptions {
        dockerfile: "Dockerfile".to_string(),
        t: "vscode-schema-server".to_string(),
        ..Default::default()
    };

    let mut image_build_stream =
        docker.build_image(build_image_options, None, Some(compressed.into()));

    // while let Some(msg) = image_build_stream.next().await {
    //     println!("Message: {:?}", msg);
    // }
    //
    // docker
    //     .build_image(build_image_options, None, Some(compressed.into()))
    //     .try_collect::<Vec<_>>()
    //     .await?;

    let create_container_options = CreateContainerOptions {
        name: "vscode-schema-server".to_string(),

        ..Default::default()
    };

    let port_binding = PortBinding {
        host_ip: Some("0.0.0.0".to_string()),
        host_port: Some("8000".to_string()),
    };

    let mut port_bindings: Vec<PortBinding> = Vec::new();
    port_bindings.push(port_binding);
    let mut port_map: PortMap = HashMap::new();
    port_map.insert("8000".to_string(), Some(port_bindings));

    set_default_env("GITHUB_ACTIONS", "false");
    let github_actions = env::var("GITHUB_ACTIONS").expect("GITHUB_ACTIONS not set");
    
    let volume = if github_actions == "true" {
        "/home/runner/work/vscode-schemas/vscode-schemas:/root/vscode-schemas".to_string()
    } else {
        let dir = env::var("VS_SCHEMAS_DIR").expect("VS_SCHEMAS_DIR not set");
        format!("{}:/root/vscode-schemas", dir)
    };

    let id = docker
        .create_container(
            Some(create_container_options),
            Config {
                image: Some("vscode-schema-server".to_string()),
                hostname: Some("vscode".to_string()),
                host_config: Some(HostConfig {
                    network_mode: Some("host".to_string()),
                    port_bindings: Some(port_map),
                    binds: Some(vec![volume]),
                    ..Default::default()
                }),
                ..Default::default()
            },
        )
        .await?
        .id;

    docker
        .start_container(&id, None::<StartContainerOptions<String>>)
        .await?;

    // let logs = docker
    //     .logs(
    //         "vscode-schema-server",
    //         Some(LogsOptions::<String> {
    //             stdout: true,
    //             ..Default::default()
    //         }),
    //     )
    //     .try_collect::<Vec<_>>()
    //     .await?;

    // for log in logs {
    //     println!("log: {:?}", log);
    // }

    // docker
    //     .remove_container("vscode-schema-server", None::<RemoveContainerOptions>)
    //     .await
    //     .unwrap();

    Ok(())
}
