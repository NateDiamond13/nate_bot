# Use Rust devcontainer as base (https://github.com/devcontainers/images/tree/main/src/rust)
FROM mcr.microsoft.com/devcontainers/rust:1.0.27-bookworm AS base

# Update and install packages
RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install -y cmake chromium-driver

# Update Rust toolchain and install clippy & rustfmt
RUN rustup toolchain uninstall stable \
    && rustup toolchain install stable \
    && rustup default stable \
    && rustup update stable \
    && rustup component add clippy rustfmt \
    && rustup toolchain install nightly \
    && rustup component add rustfmt --toolchain nightly

# Add sqlx-cli for sqlx
RUN cargo install sqlx-cli --no-default-features --features postgres,rustls
