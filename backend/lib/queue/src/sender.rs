use crate::SendableJob;
use crate::broker::RedisQueueBroker;
use crate::prelude::Result;
use crate::redis_queue::QueueItem;

const TASK_MAX_RETRIES: usize = 2;
const TASK_TIMEOUT_SECS: u64 = 3600;

#[derive(Clone, Debug)]
pub struct QueueSender {
    broker: RedisQueueBroker,
    max_retries: usize,
    timeout_secs: u64,
}

impl QueueSender {
    /// Create a new [`QueueSender`] to send jobs to the queues (with default `max_retries`)
    pub fn new(broker_url: impl Into<String>) -> Result<Self> {
        let broker = RedisQueueBroker::new(broker_url)?;

        Ok(Self {
            broker,
            max_retries: TASK_MAX_RETRIES,
            timeout_secs: TASK_TIMEOUT_SECS,
        })
    }

    pub async fn send_job(&self, job: &SendableJob) -> Result<bool> {
        let mut conn_manager = self.broker.get_connection_manager().await?;

        let item_id = format!("{job:?}");
        let queue_item: QueueItem<SendableJob> =
            QueueItem::new(job.clone(), item_id, self.max_retries, self.timeout_secs);

        self.broker.add_item(&mut conn_manager, &queue_item).await
    }
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use crate::prelude::Result;
    use crate::sender::{QueueSender, SendableJob};

    #[ignore]
    #[test(tokio::test)]
    async fn test_send_job() -> Result<()> {
        let broker_url = utils::get_config().redis_url;
        let sender = QueueSender::new(&broker_url)?;

        let result = sender.send_job(&SendableJob::PatchScraper).await;

        assert!(result.is_ok());
        Ok(())
    }
}
