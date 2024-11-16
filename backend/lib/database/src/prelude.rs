//! Library - Database prelude

/// Database library error
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

/// Database library result
pub type Result<T> = core::result::Result<T, Error>;
