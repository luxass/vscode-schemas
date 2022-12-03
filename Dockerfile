
FROM buildpack-deps:20.04-curl

RUN apt-get update && apt-get install -y --no-install-recommends \
    git \
    build-essential \
    libsecret-1-dev \
    libx11-dev \
    libxkbfile-dev \
    sudo

# Install Node.js
RUN curl -sL https://deb.nodesource.com/setup_16.x | sudo -E bash - && \
      sudo apt-get install -y nodejs


# Install Yarn
RUN npm install -g yarn

ARG tag_name

# Clone VSCode Source Code
RUN git clone --depth 1 --branch $tag_name https://github.com/microsoft/vscode.git /vscode

COPY ./patches /vscode/patches

WORKDIR /vscode

# Install Yarn Dependencies
RUN yarn --frozen-lockfile install -y


CMD [ "yarn", "install", "-y" ]