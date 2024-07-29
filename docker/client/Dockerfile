# Rust alpine base image
FROM rust:1.80-alpine3.20 AS builder

# Install dependencies for songbird
RUN apk add --no-cache alpine-sdk cmake pkgconfig libressl-dev

# Create a new empty shell project
RUN USER=root cargo new --bin discord_bot
WORKDIR /discord_bot

# Copy manifests
COPY ./discord_bot/Cargo.lock .
COPY ./discord_bot/Cargo.toml .

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

# Copy binary to an alpine image
FROM alpine:3.20 AS runner
RUN apk add --no-cache yt-dlp tini
COPY --from=builder /discord_bot/target/release/discord_bot .

# Run command
CMD ["tini", "./discord_bot"]
