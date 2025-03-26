use celery::beat::{Beat, BeatBuilder, LocalSchedulerBackend};

use crate::prelude::{Error, Result};

pub type WorkerScheduler = Beat<LocalSchedulerBackend>;

pub async fn get_scheduler(
    app_name: &str,
    broker_url: &str,
    queue_name: &str,
) -> Result<WorkerScheduler> {
    let scheduler = BeatBuilder::with_default_scheduler_backend(app_name, broker_url)
        .default_queue(queue_name)
        .build()
        .await?;
    Ok(scheduler)
}

pub trait Schedulable {
    async fn start_schedule(&mut self) -> Result<()>;
}

impl Schedulable for WorkerScheduler {
    async fn start_schedule(&mut self) -> Result<()> {
        self.start().await.map_err(Error::CeleryBeat)
    }
}
