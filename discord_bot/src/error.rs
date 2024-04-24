//! Main Crate Error

#[allow(dead_code)]
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Generic Error: {0}")]
    Generic(String),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    EnvVar(#[from] std::env::VarError),

    #[error(transparent)]
    Dotenv(#[from] dotenv::Error),

    #[error(transparent)]
    Serenity(#[from] serenity::Error),
}
