//! Library - Webhooks prelude

/// Webhooks library error
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    LibDatabase(#[from] database::Error),

    #[error(transparent)]
    Serenity(#[from] serenity::Error),
}

/// Utils library result
pub type Result<T> = core::result::Result<T, Error>;
