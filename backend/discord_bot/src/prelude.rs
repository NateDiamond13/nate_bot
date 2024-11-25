//! Discord Bot prelude

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    LibDatabase(#[from] database::Error),

    #[error(transparent)]
    LibUtils(#[from] utils::Error),

    #[error("Could not parse valid command arguments")]
    CommandArgParse,

    #[error("Could not toggle role for user")]
    ToggleRoleFailure,

    #[error("Could not find valid voice channel")]
    InvalidVoiceChannel,

    #[error("Could not find valid guild")]
    InvalidGuild,

    #[error("Could not parse valid video details")]
    VideoDetailParse,

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),

    #[error(transparent)]
    Regex(#[from] regex::Error),

    #[error(transparent)]
    Serenity(#[from] serenity::Error),

    #[error(transparent)]
    SerenityToken(#[from] serenity::secrets::TokenError),

    #[error(transparent)]
    SongbirdAudioStream(#[from] songbird::input::AudioStreamError),

    #[error(transparent)]
    SongbirdJoin(#[from] songbird::error::JoinError),
}

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
