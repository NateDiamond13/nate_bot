//! Crate prelude

pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub struct CommandData {
    pub shard_count: u32,
    pub reaction_target_ids: Vec<u64>,
    pub reaction_target_odds: u32,
}

pub type Context<'a> = poise::Context<'a, CommandData, Error>;
