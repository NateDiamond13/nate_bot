//! Main Crate Error

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

    #[error("Could not find video")]
    VideoNotFound,

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
}
