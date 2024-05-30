//! Main Crate Error

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Could not find environment variable: {0}")]
    MissingVar(String),

    #[error("Environment variable '{0}' not in valid range: {1} - {2}")]
    InvalidRangeVar(String, u32, u32),

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
    RustyYTDL(#[from] rusty_ytdl::VideoError),

    #[error(transparent)]
    Serenity(#[from] serenity::Error),

    #[error(transparent)]
    SongbirdAudioStream(#[from] songbird::input::AudioStreamError),

    #[error(transparent)]
    SongbirdJoin(#[from] songbird::error::JoinError),

    #[error(transparent)]
    SQLx(#[from] sqlx::Error),
}
