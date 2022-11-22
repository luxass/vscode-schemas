use anyhow::Result;
use bollard::image::BuildImageOptions;
use bollard::Docker;

use bollard::container::{
    Config, CreateContainerOptions, KillContainerOptions, RemoveContainerOptions,
    StartContainerOptions,
};
use bollard::service::{
    ContainerCreateResponse, HostConfig, Mount, MountTypeEnum, PortBinding, PortMap,
};
use log::debug;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::io::Write;
use std::path::Path;

use futures_util::TryStreamExt;

pub struct Ducker {
    docker: Docker,
}

impl Ducker {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            docker: Docker::connect_with_socket_defaults()?,
        })
    }

    pub async fn build_image(&self, metadata_path: &Path) -> Result<(), Box<dyn Error>> {
        let dockerfile = format!(
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
        ENV VSCODE_SCHEMAS_AUTO_RUN=true 
        ENV VSCODE_SCHEMA_OUTPUT_PATH=./
        ENV VSCODE_SCHEMA_OVERWRITE_SCHEMA_LIST={metadata_path}
        
        ENTRYPOINT [ "/bin/sh", "-c", "code-server serve-local --accept-server-license-terms --disable-telemetry --without-connection-token --host 0.0.0.0 --start-server --install-extension luxass.vscode-schema-extractor" ]
        EXPOSE 8000
    "#,
            metadata_path = metadata_path.to_str().unwrap()
        );

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

        let build_info = self
            .docker
            .build_image(build_image_options, None, Some(compressed.into()))
            .try_collect::<Vec<_>>()
            .await?;

        for info in build_info {
            debug!("{:?}", info);
        }

        Ok(())
    }

    pub async fn create_container(
        &self,
        container_name: &str,
    ) -> Result<ContainerCreateResponse, Box<dyn Error>> {
        let create_container_options = CreateContainerOptions {
            name: container_name.to_string(),

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

        let github_actions = env::var("GITHUB_ACTIONS").expect("GITHUB_ACTIONS not set");

        let source = if github_actions == "true" {
            String::from("/home/runner/work/vscode-schemas/vscode-schemas")
        } else {
            let dir = env::var("VS_SCHEMAS_DIR").expect("VS_SCHEMAS_DIR not set");
            dir
        };

        let create_res = self
            .docker
            .create_container(
                Some(create_container_options),
                Config {
                    image: Some("vscode-schema-server".to_string()),
                    hostname: Some("vscode".to_string()),
                    host_config: Some(HostConfig {
                        network_mode: Some("host".to_string()),
                        port_bindings: Some(port_map),
                        // binds: Some(vec![volume]),
                        mounts: Some(vec![Mount {
                            target: Some(String::from("/root/vscode-schemas")),
                            source: Some(source),
                            typ: Some(MountTypeEnum::BIND),
                            consistency: Some(String::from("default")),
                            ..Default::default()
                        }]),
                        ..Default::default()
                    }),
                    ..Default::default()
                },
            )
            .await?;
        Ok(create_res)
    }

    pub async fn start_container(&self, id: &str) -> Result<(), Box<dyn Error>> {
        self.docker
            .start_container(id, None::<StartContainerOptions<String>>)
            .await?;
        Ok(())
    }

    pub async fn kill(&self, name: &str) -> Result<(), Box<dyn Error>> {
        let options = Some(KillContainerOptions { signal: "SIGKILL" });
        self.docker.kill_container(name, options).await?;
        Ok(())
    }

    pub async fn destroy(&self, name: &str) -> Result<(), Box<dyn Error>> {
        self.docker
            .remove_container(name, None::<RemoveContainerOptions>)
            .await?;
        Ok(())
    }
}
