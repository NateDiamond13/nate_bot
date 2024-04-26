//! Main Crate Error

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Could not find environment variable: {0}")]
    MissingVar(String),

    #[error("Environment variable '{0}' not in valid range: {1} - {2}")]
    InvalidRangeVar(String, u32, u32),

    #[error("Could not parse valid command arguments")]
    CommandArgParse,

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Regex(#[from] regex::Error),

    #[error(transparent)]
    Serenity(#[from] serenity::Error),
}
