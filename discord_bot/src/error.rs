//! Main Crate Error

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Could not find environment variable: {0}")]
    MissingVar(String),

    #[error("Could not parse valid command arguments")]
    CommandArgParse,

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Regex(#[from] regex::Error),

    #[error(transparent)]
    Serenity(#[from] serenity::Error),
}
