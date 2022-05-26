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
