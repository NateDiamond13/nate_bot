//! Library - Queue prelude

/// Queue library error
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Error for a failed job task
    #[error("Job task \"{0}\" failed with error: {1}")]
    FailedJobTask(String, String),

    /// Error for exceeding the max number of task retries
    #[error("Max retries exceeded: {0}")]
    MaxRetriesExceeded(usize),

    /// Error from [`tokio_cron_scheduler`] crate's job scheduler
    #[error(transparent)]
    JobScheduler(#[from] tokio_cron_scheduler::JobSchedulerError),

    /// Error from [`redis`] crate
    #[error(transparent)]
    Redis(#[from] redis::RedisError),

    /// Error from [`serde_json`] crate
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

/// Queue library result
pub type Result<T> = core::result::Result<T, Error>;
