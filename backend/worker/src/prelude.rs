//! Worker prelude

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    LibUtils(#[from] utils::Error),

    #[error("Could not connect to child web driver process")]
    WebDriverChild,

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    Celery(#[from] celery::error::CeleryError),

    #[error(transparent)]
    CeleryBeat(#[from] celery::error::BeatError),

    #[error(transparent)]
    CelerySchedule(#[from] celery::error::ScheduleError),
}

pub type Result<T> = core::result::Result<T, Error>;
