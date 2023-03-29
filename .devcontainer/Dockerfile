FROM rust:bullseye

RUN apt-get update && apt-get -y upgrade

# SHELL ["/bin/bash", "--login", "-c"]
RUN rustup component add rustfmt
RUN rustup component add clippy 