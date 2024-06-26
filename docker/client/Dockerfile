# Rust alpine base image
FROM rust:1.79-alpine3.20 as builder

# Install dependencies for songbird
RUN apk add --update alpine-sdk cmake pkgconfig libressl-dev

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

# Copy binary to an alpine image
FROM alpine:3.20 AS runner
RUN apk add --update yt-dlp
COPY --from=builder /discord_bot/target/release/discord_bot .

# Run command
CMD ["./discord_bot"]
