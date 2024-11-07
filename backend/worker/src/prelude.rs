//! Worker prelude

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    LibUtils(#[from] utils::Error),

    #[error(transparent)]
    Celery(#[from] celery::error::CeleryError),

    #[error(transparent)]
    CeleryBeat(#[from] celery::error::BeatError),

    #[error(transparent)]
    CelerySchedule(#[from] celery::error::ScheduleError),
}

pub type Result<T> = core::result::Result<T, Error>;
