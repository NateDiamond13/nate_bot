use poise::command;

use crate::helpers::settings::{self, BotStatus};
use crate::prelude::{Context, Result};

/// Base command for presence, use subcommands.
#[command(
    slash_command,
    category = "Presence",
    subcommands("presence_activity", "presence_status"),
    subcommand_required,
    guild_only,
    required_permissions = "ADMINISTRATOR"
)]
pub async fn presence(_: Context<'_>) -> Result<()> {
    Ok(())
}

/// Update the bot's current activity.
#[command(slash_command, category = "Presence", rename = "activity")]
pub async fn presence_activity(
    ctx: Context<'_>,
    #[description = "Current activity"]
    #[max_length = 128]
    activity: Option<String>,
) -> Result<()> {
    match settings::update_activity(ctx.serenity_context(), &ctx.data(), activity.clone()).await {
        Ok(true) => {
            if let Some(act) = activity {
                ctx.say(format!("Bot activity updated to: \"{act}\""))
                    .await?;
            } else {
                ctx.say("Bot activity cleared").await?;
            }
        }
        _ => {
            ctx.say("Unable to update bot activity").await?;
        }
    };

    Ok(())
}

/// Set the bot's current status (online, idle, etc.).
#[command(slash_command, category = "Presence", rename = "status")]
pub async fn presence_status(
    ctx: Context<'_>,
    #[description = "Current status"] bot_status: BotStatus,
) -> Result<()> {
    match settings::update_status(ctx.serenity_context(), &ctx.data(), &bot_status).await {
        Ok(true) => {
            ctx.say(format!("Bot status updated to: {bot_status}"))
                .await?;
        }
        _ => {
            ctx.say("Unable to update bot status").await?;
        }
    };

    Ok(())
}
