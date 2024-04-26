//! Crate prelude

pub use crate::error::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub struct CommandData;

pub type Context<'a> = poise::Context<'a, CommandData, Error>;
