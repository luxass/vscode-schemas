FROM ubuntu:22.04

# hadolint ignore=DL3008
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive && apt-get install -y --no-install-recommends \
    # vscode requirements
    gnome-keyring wget curl python3-minimal ca-certificates \
    # development tools
    git build-essential \
    # clean up
    && apt-get autoremove -y && apt-get clean -y && rm -rf /var/lib/apt/lists/*

# RUN wget -O vscode-server-linux-x64.tar.gz https://update.code.visualstudio.com/commit:c3511e6c69bb39013c4a4b7b9566ec1ca73fc4d5/server-linux-x64/stable && \
#     tar -xzf vscode-server-linux-x64.tar.gz && \
#     mv -f vscode-server-linux-x64 ${SERVER_ROOT} && \
#     wget -O extractor.vsix https://github.com/DeprecatedLuxas/vscode-schemas/raw/main/schema-extractor/schema-extractor-0.0.1.vsix

RUN wget -q -O- https://aka.ms/install-vscode-server/setup.sh | sh

COPY start /usr/local/bin/start

# ENTRYPOINT [ "/bin/sh", "-c", "exec ${SERVER_ROOT}/bin/code-server --accept-server-license-terms --serve-local", "--install-extension /extractor.vsix" ]
ENTRYPOINT [ "start" ]
EXPOSE 8000