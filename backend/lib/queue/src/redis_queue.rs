use chrono::{NaiveDateTime, Utc};
use redis::{AsyncCommands, Direction};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::prelude::Result;

pub type RedisClient = redis::Client;
pub type RedisConnectionManager = redis::aio::ConnectionManager;

const POLL_TIMEOUT_SECS: f64 = 10.0;

/// Data stored within each [`QueueItem`]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueueItem<T> {
    pub id: String,
    pub data: T,
    pub max_retries: usize,
    pub current_attempt: usize,
    pub created_at: NaiveDateTime,
    pub timeout_secs: u64,
}

impl<T> QueueItem<T> {
    pub fn new(data: T, id: impl Into<String>, max_retries: usize, timeout_secs: u64) -> Self {
        let created_at = Utc::now().naive_utc();
        Self {
            id: id.into(),
            data,
            max_retries,
            current_attempt: 0,
            created_at,
            timeout_secs,
        }
    }

    pub fn increment_attempt(&mut self) -> bool {
        if self.current_attempt >= self.max_retries {
            false
        } else {
            self.current_attempt += 1;
            true
        }
    }
}

/// An item for a work queue with an `id` and associated data
struct EncodedQueueItem {
    pub id: String,
    pub data_bytes: Box<[u8]>,
}

impl EncodedQueueItem {
    /// Create an item from the encoded `data_bytes` and an `id`
    pub fn from_bytes(data_bytes: Vec<u8>, id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            data_bytes: data_bytes.into_boxed_slice(),
        }
    }

    /// Create an item from the provided `item_data`
    pub fn from_data<T: Serialize>(item_data: &QueueItem<T>) -> serde_json::Result<Self> {
        Ok(Self {
            data_bytes: serde_json::to_vec(item_data)?.into(),
            id: item_data.id.clone(),
        })
    }

    /// Returns the inner data, parsed as JSON
    pub fn to_data<T: DeserializeOwned>(&self) -> serde_json::Result<QueueItem<T>> {
        serde_json::from_slice(&self.data_bytes)
    }
}

/// A work queue backed by a Redis database
#[derive(Clone, Debug)]
pub struct WorkQueue {
    /// The key for the list of items in the queue
    main_queue_key: String,
    /// The key for the list of items being processed
    processing_key: String,
    /// The key prefix for item data entries
    item_data_key: String,
}

impl WorkQueue {
    pub fn new(queue_name: impl Into<String>) -> Self {
        let name = queue_name.into();

        Self {
            main_queue_key: format!("{name}:queue"),
            processing_key: format!("{name}:processing"),
            item_data_key: format!("{name}:item"),
        }
    }

    /// Checks if the connection is still accessible, returns an error if not
    pub async fn ping(&self, db: &mut RedisConnectionManager) -> Result<()> {
        db.ping::<String>().await?;
        Ok(())
    }

    /// Add an item to the work queue. If an item with the same id already exists, return false.
    /// If the item was added successfully, return true
    pub async fn add_item<T: Serialize>(
        &self,
        db: &mut RedisConnectionManager,
        item: &QueueItem<T>,
    ) -> Result<bool> {
        // Try to add the item if it doesn't exist
        let encoded_item = EncodedQueueItem::from_data(item)?;
        let is_added = db
            .set_nx(
                format!("{}:{}", self.item_data_key, &encoded_item.id),
                encoded_item.data_bytes.as_ref(),
            )
            .await?;

        // If it was added successfully, push its id to the end of the main queue
        if is_added {
            let items_added = db
                .lpush::<&str, &str, usize>(&self.main_queue_key, &encoded_item.id)
                .await?;
            Ok(items_added > 0)
        } else {
            Ok(false)
        }
    }

    /// Poll the work queue to find an item to process. This should be called by a worker to get work to
    /// complete. When completed, the `complete` method should be called
    pub async fn poll<T: Serialize + DeserializeOwned>(
        &self,
        db: &mut RedisConnectionManager,
    ) -> Result<Option<QueueItem<T>>> {
        loop {
            let in_processing: usize = db.llen(&self.processing_key).await?;

            // Check if the main queue is empty, and if not push the first item to processing
            if in_processing == 0
                && db
                    .blmove::<&str, &str, Option<String>>(
                        &self.main_queue_key,
                        &self.processing_key,
                        Direction::Right,
                        Direction::Left,
                        POLL_TIMEOUT_SECS,
                    )
                    .await?
                    .is_none()
            {
                return Ok(None);
            }

            // Pop the first item from processing
            let Some(item_id): Option<String> = db.rpop(&self.processing_key, None).await? else {
                log::warn!("Could not acquire first item in processing queue");
                return Ok(None);
            };

            // If we got an item, fetch the associated data
            let item_key = format!("{}:{}", self.item_data_key, &item_id);
            let item_data_bytes: Vec<u8> = match db.get(&item_key).await? {
                Some(item_data) => item_data,
                // If the item doesn't actually exist, and there's no timeout, just try again
                None => continue,
            };

            let encoded_item = EncodedQueueItem::from_bytes(item_data_bytes, &item_id);
            let mut queue_item: QueueItem<T> = encoded_item.to_data()?;

            // Try to increment the attempt
            if queue_item.increment_attempt() {
                // If it can, update the item's data and push the key to the front of processing
                let new_item = EncodedQueueItem::from_data(&queue_item)?;
                let is_added = db.set(&item_key, new_item.data_bytes.as_ref()).await?;

                if is_added {
                    let items_added = db
                        .lpush::<&str, &str, usize>(&self.processing_key, &item_id)
                        .await?;
                    if items_added > 0 {
                        return Ok(Some(queue_item));
                    } else {
                        log::info!("Could not return queue item to processing: \"{item_id}\"");
                    }
                }
            } else {
                // If it can't, remove the item's data
                log::info!(
                    "Work queue maximum attempts ({}) reached for item: \"{}\"",
                    queue_item.max_retries,
                    item_id
                );

                let items_deleted: usize = db
                    .del(format!("{}:{}", self.item_data_key, &item_id))
                    .await?;
                if items_deleted > 0 {
                    log::info!("Successfully removed queue item: \"{item_id}\"");
                } else {
                    log::error!("Could not remove queue item: \"{item_id}\"");
                }
            }
        }
    }

    /// Marks a job as completed and removes it from the main/processing queues
    pub async fn complete(&self, db: &mut RedisConnectionManager, item_id: String) -> Result<bool> {
        let (items_deleted, (), ()): (usize, (), ()) = redis::pipe()
            .del(format!("{}:{}", self.item_data_key, &item_id))
            .lrem(&self.main_queue_key, 0, &item_id)
            .lrem(&self.processing_key, 0, &item_id)
            .query_async(db)
            .await?;

        Ok(items_deleted > 0)
    }
}
