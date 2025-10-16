//! Discord Bot prelude

#[derive(thiserror::Error, Debug)]
pub enum SongbirdError {
    #[error(transparent)]
    SongbirdAudioStream(#[from] songbird::input::AudioStreamError),

    #[error(transparent)]
    SongbirdJoin(#[from] songbird::error::JoinError),
}

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

    #[error("Missing auth token for {0}")]
    MissingAuthToken(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),

    #[error(transparent)]
    Regex(#[from] regex::Error),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    ReqwestMiddleware(#[from] reqwest_middleware::Error),

    #[error(transparent)]
    Serenity(#[from] serenity::Error),

    #[error(transparent)]
    SerenityToken(#[from] serenity::secrets::TokenError),

    #[error(transparent)]
    Songbird(Box<SongbirdError>),
}

pub type Result<T> = core::result::Result<T, Error>;

pub type HttpClient = reqwest::Client;

#[derive(Clone, Debug)]
pub struct CommandData {
    pub env: utils::EnvVariables,
    pub pool: database::DbPool,
    pub http_client: HttpClient,
    pub songbird_manager: std::sync::Arc<songbird::Songbird>,
}

pub type Command = poise::Command<CommandData, Error>;

pub type Context<'a> = poise::Context<'a, CommandData, Error>;
