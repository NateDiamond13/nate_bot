use chrono::Utc;
use poise::command;
use serenity::all::{Channel, GetMessages, MessageId};

use crate::helpers;
use crate::prelude::{Context, Result};

const PURGE_LIMIT: u8 = 100;
const MAX_PURGE_DAYS: i64 = 14;

/// Purge channel of previous messages containing a given string of text. (Admin only)
#[command(slash_command, guild_only, required_permissions = "ADMINISTRATOR")]
pub async fn purge(
    ctx: Context<'_>,
    #[description = "String of text to search for in previous messages"]
    #[min_length = 3]
    text: String,
) -> Result<()> {
    let (channel_id, channel_name) = match ctx.channel().await {
        Some(Channel::Guild(channel)) => (channel.id.widen(), channel.base.name),
        Some(Channel::GuildThread(thread)) => (thread.id.widen(), thread.base.name),
        Some(_) => {
            return Ok(());
        }
        None => {
            return Ok(());
        }
    };

    let text_lower = text.to_lowercase();
    let message_filter = GetMessages::new().limit(PURGE_LIMIT);

    // Get messages less than 14 days old that contain the given text
    let messages_to_delete = channel_id.messages(ctx.http(), message_filter).await?;
    let message_ids = messages_to_delete
        .iter()
        .filter(|&msg| -msg.timestamp.signed_duration_since(Utc::now()).num_days() < MAX_PURGE_DAYS)
        .filter(|&msg| msg.content.to_lowercase().contains(&text_lower))
        .map(|msg| msg.id)
        .collect::<Vec<MessageId>>();
    channel_id
        .delete_messages(
            ctx.http(),
            message_ids.as_slice(),
            Some("Removed by purge command"),
        )
        .await?;

    let response = format!(
        "User '{}' purged {} message(s) containing \"{}\" from channel '{}'",
        ctx.author().name,
        message_ids.len(),
        text,
        channel_name
    );
    println!("{response}");
    if let Some(guild_id) = ctx.guild_id() {
        helpers::post_to_spam_channel(response, ctx.serenity_context(), guild_id).await?;
    }
    ctx.say(format!("{} message(s) purged", message_ids.len()))
        .await?;

    Ok(())
}
