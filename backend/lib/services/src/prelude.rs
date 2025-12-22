//! Library - Services prelude

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    ReqwestHeaderStringParse(#[from] reqwest::header::ToStrError),

    #[error(transparent)]
    ReqwestMiddleware(#[from] reqwest_middleware::Error),
}

pub type Result<T> = core::result::Result<T, Error>;
