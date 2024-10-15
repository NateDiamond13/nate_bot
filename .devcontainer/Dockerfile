# Use Rust devcontainer as base
FROM mcr.microsoft.com/devcontainers/rust:1.0.17-bookworm AS base

# Update pacakges and install cmake
RUN apt-get update \
    && apt-get upgrade -y \
    && apt-get install -y cmake

# Update Rust toolchain and install clippy & rustfmt
RUN rustup toolchain install stable \
    && rustup default stable \
    && rustup update stable \
    && rustup component add clippy rustfmt

# Add sqlx-cli for sqlx
RUN cargo install sqlx-cli --no-default-features --features postgres,rustls
