use crate::prelude::{Context, Result};

use poise::command;

/// Ping-pong! Use to check if bot is alive.
#[command(prefix_command, slash_command)]
pub async fn ping(ctx: Context<'_>) -> Result<()> {
    ctx.say("Pong!").await?;
    Ok(())
}
