//! Library - Services prelude

/// Services library error
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Error from [`reqwest`] crate
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    /// Error while trying to parse request header string
    #[error(transparent)]
    ReqwestHeaderStringParse(#[from] reqwest::header::ToStrError),

    /// Error from [`reqwest_middleware`] crate
    #[error(transparent)]
    ReqwestMiddleware(#[from] reqwest_middleware::Error),
}

/// Services library result
pub type Result<T> = core::result::Result<T, Error>;
