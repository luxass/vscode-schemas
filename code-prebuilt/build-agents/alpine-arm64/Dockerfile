FROM arm64v8/alpine:3.18

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

RUN npm install -g yarn

RUN mkdir -p /root/vscode
WORKDIR /root/vscode
