use bollard::image::BuildImageOptions;
use bollard::Docker;

use bollard::container::{
    Config, CreateContainerOptions, StartContainerOptions, WaitContainerOptions,
};
use std::collections::HashMap;
use std::fs;
use std::io::Write;

use futures_util::stream::StreamExt;
use futures_util::TryStreamExt;

pub fn build_dockerfile(commit: String) -> String {
    format!(
        r#"FROM buildpack-deps:20.04-curl

RUN apt-get update && apt-get install -y --no-install-recommends \
    git \
    sudo

ARG SERVER_ROOT="/home/.vscode-server"

RUN wget -O vscode-server-linux-x64.tar.gz https://update.code.visualstudio.com/commit:{commit}/server-linux-x64/stable && \
    tar -xzf vscode-server-linux-x64.tar.gz && \
    mv -f vscode-server-linux-x64 ${{SERVER_ROOT}}


WORKDIR /home/workspace/

ENV LANG=C.UTF-8 \
    LC_ALL=C.UTF-8 \
    HOME=/home/workspace \
    EDITOR=code \
    VISUAL=code \
    GIT_EDITOR="code --wait" \
    SERVER_ROOT=${{SERVER_ROOT}}

EXPOSE 5000

ENTRYPOINT [ "/bin/sh", "-c", "exec ${{SERVER_ROOT}}/bin/code-server --host 0.0.0.0 --without-connection-token --install-extension usernamehw.errorlens \"${{@}}\"", "--" ]"#,
        commit = commit
    )
}

pub async fn init(commit: String) {
    let docker = Docker::connect_with_socket_defaults().unwrap();
    let dockerfile = build_dockerfile(commit);
    let mut header = tar::Header::new_gnu();
    header.set_path("Dockerfile").unwrap();
    header.set_size(dockerfile.len() as u64);
    header.set_mode(0o755);
    header.set_cksum();
    let mut tar = tar::Builder::new(Vec::new());
    tar.append(&header, dockerfile.as_bytes()).unwrap();

    let uncompressed = tar.into_inner().unwrap();
    let mut c = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    c.write_all(&uncompressed).unwrap();
    let compressed = c.finish().unwrap();

    let build_image_options = BuildImageOptions {
        dockerfile: "Dockerfile".to_string(),
        t: "vscode-schema-server".to_string(),
        pull: true,
        rm: true,
        ..Default::default()
    };

    docker
        .build_image(build_image_options, None, Some(compressed.into()))
        .try_collect::<Vec<_>>()
        .await
        .unwrap();

    let create_container_options = CreateContainerOptions {
        name: "vscode-schema-server".to_string(),
        ..Default::default()
    };

    docker
        .create_container(
            Some(create_container_options),
            Config {
                image: Some("vscode-schema-server".to_string()),
                ..Default::default()
            },
        )
        .await
        .unwrap();

   docker
        .start_container(
            "vscode-schema-server",
            None::<StartContainerOptions<String>>,
        ).await.unwrap()

}
