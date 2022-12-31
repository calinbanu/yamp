FROM node:16.17.1-bullseye

RUN apt-get update && apt-get -y upgrade && apt-get install \
    sudo \
    libnss3 \
    libgtk-3-0 \
    libdrm2 \
    libgbm1 \
    libasound2 \
    libgl1-mesa-dev \
    libcanberra-gtk3-module \
    libcanberra-gtk3-0 \
    -yq --no-install-suggests --no-install-recommends \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

RUN echo "node:node" | chpasswd && adduser node sudo

USER node
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain stable -y
 
SHELL ["/bin/bash", "--login", "-c"]
RUN rustup component add rustfmt
RUN rustup component add clippy
RUN rustup target add wasm32-unknown-unknown
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh