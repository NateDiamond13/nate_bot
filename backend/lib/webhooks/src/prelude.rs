//! Library - Webhooks prelude

/// Serenity error for large error variant from [`serenity`] crate
#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct SerenityError(#[from] pub serenity::Error);

/// Webhooks library error
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Error from internal database library
    #[error(transparent)]
    LibDatabase(#[from] database::Error),

    /// Error from [`serenity`] crate
    #[error(transparent)]
    Serenity(Box<SerenityError>),
}

/// Webhooks library result
pub type Result<T> = core::result::Result<T, Error>;
