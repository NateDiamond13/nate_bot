use std::time::Duration;

use redis::aio::ConnectionManagerConfig;
use serde::Serialize;
use serde::de::DeserializeOwned;

use crate::prelude::Result;
use crate::redis_queue::{QueueItem, RedisClient, RedisConnectionManager, WorkQueue};

const CONNECTION_MAX_DELAY: Duration = Duration::from_millis(4_000);
const CONNECTION_MAX_RETRIES: usize = 5;
const CONNECTION_TIMEOUT: Option<Duration> = Some(Duration::from_secs(5));
const QUEUE_NAME: &str = "work_queue";

/// Broker to handle connections to the work queue in redis
#[derive(Clone, Debug)]
pub struct RedisQueueBroker {
    redis_client: RedisClient,
    work_queue: WorkQueue,
}

impl RedisQueueBroker {
    /// Create a new [`RedisQueueBroker`]
    pub fn new(broker_url: impl Into<String>) -> Result<Self> {
        let redis_client = RedisClient::open(broker_url.into())?;
        let work_queue = WorkQueue::new(QUEUE_NAME);

        Ok(Self {
            redis_client,
            work_queue,
        })
    }

    /// Get the [`RedisConnectionManager`] to handle all connections
    pub async fn get_connection_manager(&self) -> Result<RedisConnectionManager> {
        log::info!("Attempting to acquire Redis connection...");
        let config = ConnectionManagerConfig::new()
            .set_connection_timeout(CONNECTION_TIMEOUT)
            .set_max_delay(CONNECTION_MAX_DELAY)
            .set_number_of_retries(CONNECTION_MAX_RETRIES);
        let conn = self
            .redis_client
            .get_connection_manager_with_config(config)
            .await?;

        log::info!("Redis connection acquired, continuing...");

        Ok(conn)
    }

    /// Checks if the connection is still accessible, returns an error if not
    pub async fn ping(&self, conn_manager: &mut RedisConnectionManager) -> Result<()> {
        self.work_queue.ping(conn_manager).await
    }

    /// Poll the work queue for an item to work on
    pub async fn poll<T: Serialize + DeserializeOwned>(
        &self,
        conn_manager: &mut RedisConnectionManager,
    ) -> Result<Option<QueueItem<T>>> {
        self.work_queue.poll(conn_manager).await
    }

    /// Mark a [`QueueItem`] as complete, removing it from the queue
    pub async fn complete<T>(
        &self,
        conn_manager: &mut RedisConnectionManager,
        queue_item_data: QueueItem<T>,
    ) -> Result<bool> {
        self.work_queue
            .complete(conn_manager, queue_item_data.id)
            .await
    }

    /// Add a [`QueueItem`] with the given `inner_data`, `item_id`, and `max_retries` to the queue
    pub async fn add_item<T: Serialize>(
        &self,
        conn_manager: &mut RedisConnectionManager,
        queue_item: &QueueItem<T>,
    ) -> Result<bool> {
        self.work_queue.add_item(conn_manager, queue_item).await
    }
}
