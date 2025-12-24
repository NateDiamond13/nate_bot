//! Queue library to handle job queue with [`redis`]

mod broker;
mod listener;
mod prelude;
mod redis_queue;
mod scheduler;
mod sender;

pub use listener::QueueListener;
pub use prelude::*;
pub use scheduler::{QueueScheduler, ScheduledJob};
pub use sender::QueueSender;

/// Enum of job tasks that can be sent to the queue
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum SendableJob {
    /// Patch Scraper job
    PatchScraper,
}
