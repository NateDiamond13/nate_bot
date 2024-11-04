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
2. `docker run --env-file .env.dev --rm nate-bot-rust-client` to run the container
3. `docker buildx prune` to clear build cache

### SQLx

1. `sqlx migrate build-script` to generate build script (build.rs)
2. `sqlx migrate add -r <name>` to add migration (reversible)
3. `sqlx migrate run` to run all migrations
4. `sqlx migrate revert` to revert previous migration
