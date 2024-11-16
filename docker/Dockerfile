# Rust alpine base image
FROM rust:1.82-alpine3.20

# Install dependencies for crates
RUN apk add alpine-sdk=1.0-r1 \
    cmake=3.29.3-r0 \
    libressl-dev=3.8.4-r0

# Create empty shell projects that match structure
WORKDIR /backend
RUN USER=root cargo new --bin discord_bot --vcs none \
    && USER=root cargo new --bin worker --vcs none \
    && USER=root cargo new --lib lib/database --vcs none \
    && USER=root cargo new --lib lib/utils --vcs none \
    && USER=root cargo new --lib lib/webhooks --vcs none

# Copy manifests
COPY ./backend/Cargo.toml ./backend/Cargo.lock ./
COPY ./backend/discord_bot/Cargo.toml ./discord_bot/Cargo.toml
COPY ./backend/worker/Cargo.toml ./worker/Cargo.toml
COPY ./backend/lib/database/Cargo.toml ./lib/database/Cargo.toml
COPY ./backend/lib/utils/Cargo.toml ./lib/utils/Cargo.toml
COPY ./backend/lib/webhooks/Cargo.toml ./lib/webhooks/Cargo.toml

# Include database url in build for SQLx
ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL

# Build dependencies to cache them and remove build artifacts
RUN cargo build --release
RUN rm ./target/release/deps/discord_bot* \
    && rm ./target/release/deps/worker* \
    && rm ./target/release/deps/libdatabase* \
    && rm ./target/release/deps/libutils* \
    && rm ./target/release/deps/libwebhooks*
