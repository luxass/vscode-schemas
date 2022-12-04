use std::{collections::HashMap, io::Write};

use bollard::{
    image::{BuildImageOptions, ListImagesOptions},
    Docker,
};
use futures_util::TryStreamExt;
use log::trace;

use crate::errors::Error;

pub async fn build_code_image(docker: &Docker, release: &String) -> Result<(), Error> {
    // let mut file = std::fs::File::open("Dockerfile")?;
    // let mut dockerfile = String::new();
    // file.read_to_string(&mut dockerfile)?;

    let dockerfile = r#"
FROM buildpack-deps:20.04-curl

RUN apt-get update && apt-get install -y --no-install-recommends \
    git \
    build-essential \
    libsecret-1-dev \
    libx11-dev \
    libxkbfile-dev \
    libnss3 \
    libatk1.0-0 \
    sudo

# Install Node.js
RUN curl -sL https://deb.nodesource.com/setup_16.x | sudo -E bash - && \
      sudo apt-get install -y nodejs


# Install Yarn
RUN npm install -g yarn

ARG tag_name

# Clone VSCode Source Code
RUN git clone --depth 1 --branch $tag_name https://github.com/microsoft/vscode.git /vscode


# Clone VSCode Schemas Source Code
RUN git clone --depth 1 --branch v2 https://github.com/luxass/vscode-schemas.git /vscode-schemas

# Move Patches to VSCode
RUN mv /vscode-schemas/patches /vscode/patches

WORKDIR /vscode

# Apply Patches
RUN patch -u src/vs/platform/windows/electron-main/windowImpl.ts -i patches/windowImpl.patch

# Install Yarn Dependencies
RUN yarn --frozen-lockfile install -y

# Build VSCode
RUN yarn compile

# Install Extension
RUN ./scripts/code.sh --install-extension luxass.vscode-schema-extractor

CMD [ "./scripts/code.sh" ]
    "#;

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

    let tag = format!("vscode-build-agent:{}", release);

    let mut buildargs = HashMap::new();
    buildargs.insert(String::from("tag_name"), release.to_string());

    let build_image_options = BuildImageOptions {
        dockerfile: "Dockerfile".to_string(),
        t: tag,
        buildargs,
        ..Default::default()
    };

    let build_info = docker
        .build_image(build_image_options, None, Some(compressed.into()))
        .try_collect::<Vec<_>>()
        .await?;

    for info in build_info {
        if info.stream.is_some() {
            if let Some(stream) = info.stream {
                trace!("{}", stream);
            }
        }
    }
    Ok(())
}

pub async fn check_for_image(docker: &Docker, image_tag: String) -> Result<bool, Error> {
    let mut filters = HashMap::new();
    filters.insert("dangling", vec!["false"]);

    let options = ListImagesOptions {
        all: true,
        filters,
        ..Default::default()
    };
    let images = docker.list_images(Some(options)).await?;

    for image in images {
        for tag in image.repo_tags {
            if tag == image_tag {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
