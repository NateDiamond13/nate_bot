//! Services - Library
//!
//! Internal library to handle interacting with external API services (Steam, etc.)

mod client;
mod prelude;
mod steam;

pub use client::RequestClient;
pub use prelude::*;
pub use steam::{SteamAppNewsItem, SteamClient};
