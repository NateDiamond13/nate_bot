//! Library - Utils prelude

/// Utils library error
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Could not find environment variable: {0}")]
    MissingVar(String),

    #[error("Environment variable '{0}' not in valid range: {1} - {2}")]
    InvalidRangeVar(String, u32, u32),
}

/// Utils library result
pub type Result<T> = core::result::Result<T, Error>;
