# syntax = devthefuture/dockerfile-x

# Use base image for all builds
FROM ./docker/Dockerfile AS builder

# Copy src files
COPY ./backend/discord_bot/src ./discord_bot/src
COPY ./backend/lib/database/src ./lib/database/src
COPY ./backend/lib/utils/src ./lib/utils/src
COPY ./backend/lib/webhooks/src ./lib/webhooks/src

# Build for release
RUN cargo build --release -p discord_bot

# Copy binary to an alpine image
FROM alpine:3.21 AS runner
RUN apk upgrade --no-cache \
    && apk add --no-cache yt-dlp tini
COPY --from=builder /backend/target/release/discord_bot .

# Run command
CMD ["tini", "./discord_bot"]
