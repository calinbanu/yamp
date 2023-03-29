FROM rust:bullseye

RUN apt-get update && apt-get -y upgrade && apt-get install \
    sudo \
    libwebkit2gtk-4.0-dev \
    build-essential \
    curl \
    wget \
    gdb \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    libcanberra-gtk3-module \
    libcanberra-gtk3-0 \
    -yq --no-install-suggests --no-install-recommends \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

RUN useradd -rm -d /home/tauri -s /bin/bash -U -G sudo -u 1000 -p "$(openssl passwd -6 tauri)" tauri
RUN git clone https://github.com/pwndbg/pwndbg /home/tauri/pwndbg && cd /home/tauri/pwndbg && ./setup.sh

USER tauri

# SHELL ["/bin/bash", "--login", "-c"]
RUN rustup component add rustfmt
RUN rustup component add clippy
RUN rustup target add wasm32-unknown-unknown
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN cargo install create-tauri-app
RUN cargo install tauri-cli
RUN cargo install trunk