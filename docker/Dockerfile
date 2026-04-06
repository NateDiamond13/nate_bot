# Rust alpine base image with cargo-chef (https://hub.docker.com/r/lukemathwalker/cargo-chef)
FROM lukemathwalker/cargo-chef:0.1.77-rust-1.94.1-alpine3.23 AS chef
WORKDIR /backend

# Plan and prepare build recipe
FROM chef AS planner
COPY ./backend .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /backend/recipe.json recipe.json

# Install dependencies for crates (https://pkgs.alpinelinux.org/packages)
RUN apk add alpine-sdk=1.1-r0 \
  libressl-dev=4.2.1-r0

# Set environment variable to build sqlx offline
ENV SQLX_OFFLINE=true

# Build cached dependencies from recipe
RUN cargo chef cook --release --recipe-path recipe.json
