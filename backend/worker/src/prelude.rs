//! Worker prelude

/// Worker error
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Could not connect to child web driver process")]
    WebDriverChild,

    #[error("WebDriverFailure: `{0}`")]
    WebDriverInternal(String),

    #[error(transparent)]
    LibDatabase(#[from] database::Error),

    #[error(transparent)]
    LibQueue(#[from] queue::Error),

    #[error(transparent)]
    LibUtils(#[from] utils::Error),

    #[error(transparent)]
    LibWebhooks(#[from] webhooks::Error),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Regex(#[from] regex::Error),
}

/// Worker result
pub type Result<T> = core::result::Result<T, Error>;

/// Queue library error
pub type QueueError = queue::Error;

/// Queue library result
pub type QueueResult<T> = core::result::Result<T, QueueError>;
