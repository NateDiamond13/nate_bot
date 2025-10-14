//! Library - Queue prelude

/// Queue library error
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Job task \"{0}\" failed with error: {1}")]
    FailedJobTask(String, String),

    #[error("Max retries exceeded: {0}")]
    MaxRetriesExceeded(usize),

    #[error(transparent)]
    JobScheduler(#[from] tokio_cron_scheduler::JobSchedulerError),

    #[error(transparent)]
    Redis(#[from] redis::RedisError),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::error::Error),
}

/// Queue library result
pub type Result<T> = core::result::Result<T, Error>;
