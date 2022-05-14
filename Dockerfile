FROM buildpack-deps:20.04-curl

RUN apt-get update && apt-get install -y --no-install-recommends \
    git \
    sudo

