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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub enum SendableJob {
    PatchScraper,
}
