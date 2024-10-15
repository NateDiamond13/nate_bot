//! Crate prelude

pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

pub type HttpClient = reqwest::Client;

#[derive(Debug, Clone)]
pub struct CommandData {
    pub env: utils::EnvVariables,
    pub pool: database::PgPool,
    pub http_client: HttpClient,
    pub songbird_manager: std::sync::Arc<songbird::Songbird>,
}

pub type Context<'a> = poise::Context<'a, CommandData, Error>;
