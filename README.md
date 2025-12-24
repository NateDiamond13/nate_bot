# NateBot

NateBot for Discord, written in Rust

## Useful Commands

### Cargo

1. `cargo build --workspace` to build all parts of the project
2. `cargo run` to run the project
3. `cargo test` to run the tests
4. `cargo update` to update all packages and their dependencies
5. `cargo update --recursive <package>` to update specific package and its dependencies

#### Cargo Formatting (Nightly / Unstable)

1. `rustup toolchain install nightly` to install the nightly toolchain
2. `cargo +nightly fmt` to run rustfmt with unstable features

### Docker

1. `docker compose build` to build the image
2. `docker run --env-file .env --rm nate_bot-discord_bot` to run the bot container
3. `docker run --env-file .env --rm nate_bot-worker` to run the worker container
4. `docker buildx prune` to clear build cache
5. `docker image pull redis:alpine` to pull latest Redis image
6. `docker run -p 6379:6379 --name my-redis -d --rm redis:alpine` to start background Redis server

### SQLx

1. `cargo sqlx migrate build-script` to generate build script (build.rs)
2. `cargo sqlx migrate add -r <name>` to add migration (reversible)
3. `cargo sqlx migrate run` to run all migrations
4. `cargo sqlx migrate revert` to revert previous migration
5. `cargo sqlx prepare` within backend/lib/database directory to prepare query statements
