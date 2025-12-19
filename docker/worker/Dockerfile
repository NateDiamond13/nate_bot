# syntax = devthefuture/dockerfile-x

# Use base image for all builds
FROM ./docker/Dockerfile AS builder

# Copy src files
COPY ./backend/worker/src ./worker/src
COPY ./backend/lib/database/src ./lib/database/src
COPY ./backend/lib/queue/src ./lib/queue/src
COPY ./backend/lib/utils/src ./lib/utils/src
COPY ./backend/lib/webhooks/src ./lib/webhooks/src

# Copy sqlx files
COPY ./backend/lib/database/.sqlx ./lib/database/.sqlx

# Build for release
RUN cargo build --release -p worker

# Copy binary to an alpine image
FROM alpine:3.23 AS runner
RUN apk upgrade --no-cache \
    && apk add --no-cache tini chromium-chromedriver
COPY --from=builder /backend/target/release/worker .

# Run command
CMD ["tini", "./worker"]
