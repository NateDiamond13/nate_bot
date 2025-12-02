# Rust alpine base image with cargo-chef (https://hub.docker.com/r/lukemathwalker/cargo-chef)
FROM lukemathwalker/cargo-chef:0.1.73-rust-1.91.1-alpine3.22 AS chef
WORKDIR /backend

# Plan and prepare build recipe
FROM chef AS planner
COPY ./backend .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /backend/recipe.json recipe.json

# Install dependencies for crates (https://pkgs.alpinelinux.org/packages)
RUN apk add alpine-sdk=1.1-r0 \
    cmake=3.31.7-r1 \
    libressl-dev=4.1.2-r0

# Include database url in build for SQLx
ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL

# Build cached dependencies from recipe
RUN cargo chef cook --release --recipe-path recipe.json
