//! Crate prelude

pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

pub type HttpClient = reqwest::Client;

#[derive(Debug, Clone)]
pub struct CommandData {
    pub env: EnvVariables,
    pub pool: sqlx::PgPool,
    pub http_client: HttpClient,
    pub songbird_manager: std::sync::Arc<songbird::Songbird>,
}

#[derive(Debug, Clone)]
pub struct EnvVariables {
    pub audit_enabled_servers: Vec<u64>,
    pub command_prefix: String,
    pub custom_status: String,
    pub database_url: String,
    pub discord_token: String,
    pub lottery_odds: u32,
    pub queue_max_sounds: usize,
    pub reaction_target_ids: Vec<u64>,
    pub reaction_target_odds: u32,
}

pub type Context<'a> = poise::Context<'a, CommandData, Error>;
