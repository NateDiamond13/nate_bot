# syntax = devthefuture/dockerfile-x

# Use base image for all builds
FROM ./docker/Dockerfile AS builder

# Copy src files
COPY ./backend/worker/src ./worker/src
COPY ./backend/lib/utils/src ./lib/utils/src

# Build for release
RUN cargo build --release -p worker

# Copy binary to an alpine image
FROM alpine:3.20 AS runner
RUN apk add --no-cache yt-dlp tini chromium-chromedriver
COPY --from=builder /backend/target/release/worker .

# Run command
CMD ["tini", "./worker"]
