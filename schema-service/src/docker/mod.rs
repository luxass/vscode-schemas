use bollard::image::BuildImageOptions;
use bollard::Docker;

use std::collections::HashMap;
use std::fs;

use futures_util::stream::StreamExt;

pub fn build_image(commit: String) {
    let docker_file = r#"
FROM buildpack-deps:20.04-curl

RUN apt-get update && apt-get install -y --no-install-recommends \
    git \
    sudo

ARG URL="https://code.visualstudio.com/sha/download?build=stable&os=linux-x64"
ARG SERVER_ROOT="/home/.vscode-server"

RUN wget https://code.visualstudio.com/sha/download?build=stable&os=linux-x64 && \
    tar -xzf code-stable-x64-1652813090.tar.gz && \
     mv -f VSCode-linux-x64 ${SERVER_ROOT} && \

WORKDIR /home/workspace/

ENV LANG=C.UTF-8 \
    LC_ALL=C.UTF-8 \
    HOME=/home/workspace \
    EDITOR=code \
    VISUAL=code \
    GIT_EDITOR="code --wait" \
    SERVER_ROOT=${SERVER_ROOT}

EXPOSE 5000

ENTRYPOINT [ "/bin/sh", "-c", "exec ${SERVER_ROOT}/code --host 0.0.0.0 --without-connection-token \"${@}\"", "--" ]
"#;

    fs::write("../Dockerfile", docker_file).expect("unable to write Dockerfile");
}

pub async fn lmao() {
    let docker = Docker::connect_with_socket_defaults().unwrap();

    let mut build_image_args = HashMap::new();
    build_image_args.insert("dummy", "value");

    let mut build_image_labels = HashMap::new();
    build_image_labels.insert("maintainer", "somemaintainer");

    let build_image_options = BuildImageOptions {

        dockerfile: "Dockerfile",
        t: "bollard-build-example",
        extrahosts: Some("myhost:127.0.0.1"),
        remote:
        "https://raw.githubusercontent.com/docker-library/openjdk/master/11/jdk/slim-buster/Dockerfile",
        q: false,
        nocache: false,
        cachefrom: vec![],
        pull: true,
        rm: true,
        forcerm: true,
        memory: Some(120000000),
        memswap: Some(120000000),

        cpushares: Some(2),
        cpusetcpus: "0-3",
        cpuperiod: Some(2000),
        cpuquota: Some(1000),
        buildargs: build_image_args,
        shmsize: Some(1000000),
        squash: false,
        labels: build_image_labels,
        networkmode: "host",
        platform: "linux/x86_64",
    };

    let mut image_build_stream = docker.build_image(build_image_options, None, None);

    while let Some(msg) = image_build_stream.next().await {
        println!("Message: {:?}", msg);
    }
}
