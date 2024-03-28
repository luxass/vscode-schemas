FROM alpine:3.19


# install necessary packages
RUN apk add \
    nodejs \
    npm \
    g++ \
    python3 \
    make \
    git \
    bash \
    curl \
    perl \
    pkgconfig \
    libsecret-dev \
    krb5-dev \
    jq

ARG NODE_VERSION="16.20.2"

# setup nodejs repo
RUN curl -L -O https://unofficial-builds.nodejs.org/download/release/v${NODE_VERSION}/node-v${NODE_VERSION}-linux-x64-musl.tar.gz && \
    tar -xzf node-v${NODE_VERSION}-linux-x64-musl.tar.gz && \
    cd node-v${NODE_VERSION}-linux-x64-musl && \
    cp -R * /usr/local/ && \
    cd .. && \
    rm -rf node-v${NODE_VERSION}-linux-x64-musl* && \
    node --version

# install yarn, node-gyp
RUN npm install -g yarn node-gyp

RUN mkdir -p /root/vscode
WORKDIR /root/vscode
