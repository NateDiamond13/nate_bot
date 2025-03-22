use poise::command;

use crate::prelude::{Context, Result};

/// Ping-pong! Use to check if bot is alive.
#[command(slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<()> {
    ctx.say("Pong!").await?;
    Ok(())
}
