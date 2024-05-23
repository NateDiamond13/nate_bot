# Rust slim base image
FROM rust:1.78-slim-buster as builder

# Install dependencies for songbird
RUN apt-get update && apt-get install -y ffmpeg cmake pkg-config libssl-dev

# Create a new empty shell project
RUN USER=root cargo new --bin discord_bot
WORKDIR /discord_bot

# Copy manifests
COPY ./discord_bot/Cargo.lock ./Cargo.lock
COPY ./discord_bot/Cargo.toml ./Cargo.toml

# Include database url in build for SQLx
ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL

# Build dependencies to cache them (remove default project source code)
RUN cargo build --release
RUN rm src/*.rs

# Copy the source and build for release
COPY ./discord_bot/src ./src
RUN rm ./target/release/deps/discord_bot*
RUN cargo build --release

# Copy binary to a slim image
FROM debian:buster-slim AS runner
RUN apt-get update && apt-get install -y ffmpeg cmake pkg-config libssl-dev
COPY --from=builder /discord_bot/target/release/discord_bot .

# Run command
CMD ["./discord_bot"]
