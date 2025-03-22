use std::sync::Arc;

use celery::{Celery, CeleryBuilder};

use crate::prelude::{Error, Result};

pub type WorkerListener = Arc<Celery>;

pub async fn get_listener(broker_url: &str, queue_name: &str) -> Result<WorkerListener> {
    let listener = CeleryBuilder::new("celery", broker_url)
        .default_queue(queue_name)
        .prefetch_count(1) // Only do one task at a time
        .task_max_retries(2) // Retry failed tasks at most 2 times
        .task_min_retry_delay(1) // Wait at least 1 second before retrying a failed task
        .task_time_limit(3600) // Tasks auto-fail after 3600 seconds (1 hour)
        .build()
        .await?;
    Ok(Arc::new(listener))
}

pub trait Listenable {
    async fn start_listen(&self) -> Result<()>;
}

impl Listenable for WorkerListener {
    async fn start_listen(&self) -> Result<()> {
        self.display_pretty().await;
        self.consume().await.map_err(Error::Celery)
    }
}
