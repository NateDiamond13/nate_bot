# Rust alpine base image
FROM rust:1.81-alpine3.20

# Install dependencies for crates
RUN apk add --no-cache alpine-sdk cmake pkgconfig libressl-dev

# Create empty shell projects that match structure
WORKDIR /backend
RUN USER=root cargo new --bin discord_bot --vcs none \
    && USER=root cargo new --lib lib/database --vcs none \
    && USER=root cargo new --lib lib/utils --vcs none

# Copy manifests
COPY ./backend/Cargo.toml ./backend/Cargo.lock ./
COPY ./backend/discord_bot/Cargo.toml ./discord_bot/Cargo.toml
COPY ./backend/lib/database/Cargo.toml ./lib/database/Cargo.toml
COPY ./backend/lib/utils/Cargo.toml ./lib/utils/Cargo.toml

# Include database url in build for SQLx
ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL

# Build dependencies to cache them and remove build artifacts
RUN cargo build --release
RUN rm ./target/release/deps/discord_bot* \
    && rm ./target/release/deps/libdatabase* \
    && rm ./target/release/deps/libutils*
