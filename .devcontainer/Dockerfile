ARG VARIANT="bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/rust:1-${VARIANT}

RUN apt-get update

# For building
RUN apt-get install -y \
  git \
  pkg-config \
  libsystemd-dev \
  build-essential \
  libelf-dev \
  libseccomp-dev \
  libclang-dev \
  libssl-dev

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y 
RUN rustup component add rustfmt
RUN rustup component add clippy 
