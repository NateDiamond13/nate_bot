# NateBot (Rust)

NateBot for Discord, written in Rust

## Useful Commands

### Cargo

1. `cargo build` to build the project
2. `cargo run` to run the project
3. `cargo test` to run the tests
4. `cargo update` to update dependencies

### Docker

1. `docker compose build` to build the image
2. `docker run --env-file .env.dev --rm nate-bot-rust-discord_bot` to run the bot container
3. `docker run --env-file .env.dev --rm nate-bot-rust-worker` to run the worker container
4. `docker buildx prune` to clear build cache
5. `docker image pull redis:alpine` to pull latest Redis image
6. `docker run -p 6379:6379 --name my-redis -d --rm redis:alpine` to start background Redis server

### SQLx

1. `sqlx migrate build-script` to generate build script (build.rs)
2. `sqlx migrate add -r <name>` to add migration (reversible)
3. `sqlx migrate run` to run all migrations
4. `sqlx migrate revert` to revert previous migration
