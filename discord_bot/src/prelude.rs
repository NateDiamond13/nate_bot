//! Crate prelude

pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone)]
pub struct CommandData {
    pub env: EnvVariables,
    pub pool: sqlx::PgPool,
}

#[derive(Debug, Clone)]
pub struct EnvVariables {
    pub command_prefix: String,
    pub custom_status: String,
    pub database_url: String,
    pub discord_token: String,
    pub lottery_odds: u32,
    pub reaction_target_ids: Vec<u64>,
    pub reaction_target_odds: u32,
    pub shard_count: u32,
}

pub type Context<'a> = poise::Context<'a, CommandData, Error>;
