FROM buildpack-deps:20.04-curl

RUN apt-get update && apt-get install -y --no-install-recommends \
    git \
    sudo

ARG SERVER_ROOT="/home/.vscode-server"

RUN wget https://update.code.visualstudio.com/commit:c3511e6c69bb39013c4a4b7b9566ec1ca73fc4d5/server-linux-x64/stable && \
    tar -xzf vscode-server-linux-x64.tar.gz && \
     mv -f vscode-server-linux-x64 ${SERVER_ROOT} && \


WORKDIR /home/workspace/

ENV LANG=C.UTF-8 \
    LC_ALL=C.UTF-8 \
    HOME=/home/workspace \
    EDITOR=code \
    VISUAL=code \
    GIT_EDITOR="code --wait" \
    SERVER_ROOT=${SERVER_ROOT}

EXPOSE 5000

ENTRYPOINT [ "/bin/sh", "-c", "exec ${SERVER_ROOT}/bin/code-server --host 0.0.0.0 --without-connection-token \"${@}\"", "--" ]