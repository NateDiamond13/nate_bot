# Use Rust devcontainer as base (https://github.com/devcontainers/images/tree/main/src/rust)
FROM mcr.microsoft.com/devcontainers/rust:2.0.6-bookworm AS base

# Update and install packages
RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install -y chromium-driver

# Update Rust toolchain and install clippy & rustfmt
RUN rustup toolchain uninstall stable \
    && rustup toolchain install stable \
    && rustup default stable \
    && rustup update stable \
    && rustup toolchain install nightly

# Add sqlx-cli for sqlx
RUN cargo install sqlx-cli --no-default-features --features postgres,rustls
