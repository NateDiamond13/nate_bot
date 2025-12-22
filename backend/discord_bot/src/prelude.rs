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
}

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub struct CommandData {
    pub env: utils::EnvVariables,
    pub pool: database::DbPool,
}

pub type Command = poise::Command<CommandData, Error>;

pub type Context<'a> = poise::Context<'a, CommandData, Error>;
