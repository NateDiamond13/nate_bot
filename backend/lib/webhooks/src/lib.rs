//! Webhooks - Library
//!
//! Internal library to handle creating and sending Discord webhooks

mod patch_notes;
mod prelude;

pub use patch_notes::{create_patch_embed, send_all_patch_alerts};
pub use prelude::*;
