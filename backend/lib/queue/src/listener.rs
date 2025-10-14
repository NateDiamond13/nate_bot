use std::time::Duration;

use tokio::time;

use crate::SendableJob;
use crate::broker::RedisQueueBroker;
use crate::prelude::{Error, Result};

#[derive(Clone, Debug)]
pub struct QueueListener {
    broker: RedisQueueBroker,
}

impl QueueListener {
    pub fn new(broker_url: impl Into<String>) -> Result<Self> {
        let broker = RedisQueueBroker::new(broker_url)?;

        Ok(Self { broker })
    }

    pub async fn start_listen<F>(&self, job_func: F) -> Result<()>
    where
        F: AsyncFn(&SendableJob) -> Result<bool>,
    {
        let mut conn_manager = self.broker.get_connection_manager().await?;
        log::info!("Starting queue listener...");

        loop {
            match self.broker.poll::<SendableJob>(&mut conn_manager).await {
                Ok(Some(job_item)) => {
                    log::info!(
                        "Trying job task \"{:?}\" ({}/{})...",
                        job_item.data,
                        job_item.current_attempt,
                        job_item.max_retries
                    );

                    // Attempt to run the job within the allotted time
                    match time::timeout(
                        Duration::from_secs(job_item.timeout_secs),
                        job_func(&job_item.data),
                    )
                    .await
                    {
                        Ok(job_result) => {
                            if job_result.is_ok() {
                                log::info!("Finished running job task \"{:?}\"", job_item.data);
                                self.broker.complete(&mut conn_manager, job_item).await?;
                            }
                        }
                        Err(_) => {
                            log::warn!(
                                "Job task \"{:?}\" timed out after {} secs",
                                job_item.data,
                                job_item.timeout_secs
                            );
                        }
                    }
                }
                Ok(None) => {
                    // Nothing currently in the queue
                }
                Err(Error::Redis(redis_err)) => {
                    if redis_err.is_unrecoverable_error() {
                        log::error!("Listener redis error: {redis_err:?}");
                        log::warn!("Listener connection lost, attempting to reconnect...");

                        match self.broker.ping(&mut conn_manager).await {
                            Ok(()) => {
                                log::info!("Listener successfully reconnected");
                            }
                            Err(err) => {
                                return Err(err);
                            }
                        }
                    } else {
                        return Err(Error::Redis(redis_err));
                    }
                }
                Err(err) => {
                    return Err(err);
                }
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use crate::SendableJob;
    use crate::listener::QueueListener;
    use crate::prelude::{Error, Result};

    async fn test_func(job: &SendableJob) -> Result<bool> {
        log::info!("Running test job: {job:?}");
        let delay = true;

        if delay {
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            Ok(true)
        } else {
            Err(Error::FailedJobTask("a".to_string(), "b".to_string()))
        }
    }

    #[ignore]
    #[test(tokio::test)]
    async fn test_listener_connection() -> Result<()> {
        let broker_url = utils::get_config().redis_url;
        let listener = QueueListener::new(&broker_url)?;
        listener.start_listen(test_func).await?;

        Ok(())
    }
}
