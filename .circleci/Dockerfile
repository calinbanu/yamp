FROM rust:1.70.0-slim-bullseye

ARG VERSION=0.0.0
LABEL version=$VERSION

RUN curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg \
    && chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg \
    && echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | tee /etc/apt/sources.list.d/github-cli.list > /dev/null

# Requirements
# codecov -> coreutils, gnupg, curl
# cargo-audit -> libssl-dev, pkg-config
# xlsxwriter -> clang
RUN apt-get update && apt-get -y upgrade && apt-get install -y git libssl-dev pkg-config clang curl coreutils gnupg zip gh

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


