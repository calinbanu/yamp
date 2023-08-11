FROM rust:1.70.0-slim-bullseye

ARG VERSION=0.0.0
LABEL version=$VERSION

# Requirements
# codecov -> coreutils, gnupg, curl
# cargo-audit -> libssl-dev, pkg-config
RUN apt-get update && apt-get -y upgrade && apt-get install -y git libssl-dev pkg-config curl coreutils gnupg zip

# Install github CLI for release creation
RUN curl -L https://github.com/cli/cli/releases/download/v2.32.1/gh_2.32.1_linux_amd64.deb > gh_2.32.1_linux_amd64.deb && dpkg -i *.deb && apt install -f

RUN rustup component add rustfmt
RUN rustup component add clippy
#  Needed for grcov
RUN rustup component add llvm-tools-preview

# nightly is necessary for generating xml unit tests
RUN rustup toolchain install nightly

# Used for security audit, see security job in circleci config.yml
RUN cargo install cargo-audit
# Used for coverage
RUN cargo install grcov
# Used for converting json to xml unit tests
RUN cargo install cargo2junit


