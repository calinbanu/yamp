FROM rust:bullseye

RUN apt-get update && apt-get -y upgrade

RUN apt-get install -y clang

# SHELL ["/bin/bash", "--login", "-c"]
RUN rustup component add rustfmt
RUN rustup component add clippy

RUN cargo install cargo-watch