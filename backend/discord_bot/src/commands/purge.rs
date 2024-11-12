use crate::helpers;
use crate::prelude::{Context, Result};

use chrono::Utc;
use poise::{command, PrefixContext};
use serenity::all::{GetMessages, MessageId};

const MIN_TEXT_LENGTH: usize = 3;
const PURGE_LIMIT: u8 = 100;

/// Purge channel of previous messages containing a given string of text. (Admin only)
#[command(
    prefix_command,
    slash_command,
    guild_only,
    required_permissions = "ADMINISTRATOR"
)]
pub async fn purge(
    ctx: Context<'_>,
    #[description = "String of text to search for in previous messages"]
    #[min_length = 3]
    text: String,
) -> Result<()> {
    if text.len() < MIN_TEXT_LENGTH {
        ctx.say(format!(
            "Text must be at least {} characters long",
            MIN_TEXT_LENGTH
        ))
        .await?;
        return Ok(());
    }
    let text_lower = text.to_lowercase();

    let channel = match ctx.guild_channel().await {
        Some(channel) => channel,
        None => {
            return Ok(());
        }
    };

    let current_message = match ctx {
        Context::Prefix(PrefixContext { msg, .. }) => Some(msg),
        _ => None,
    };

    let message_filter;
    if let Some(msg) = current_message {
        message_filter = GetMessages::new().limit(PURGE_LIMIT).before(msg.id);
    } else {
        message_filter = GetMessages::new().limit(PURGE_LIMIT);
    }

    // Get messages less than 14 days old that contain the given text
    let messages_to_delete = channel.id.messages(ctx.http(), message_filter).await?;
    let message_ids = messages_to_delete
        .iter()
        .filter(|&msg| -msg.timestamp.signed_duration_since(Utc::now()).num_days() < 14)
        .filter(|&msg| msg.content.to_lowercase().contains(&text_lower))
        .map(|msg| msg.id)
        .collect::<Vec<MessageId>>();
    channel
        .id
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
        channel.name
    );
    println!("{response}");
    if let Some(guild_id) = ctx.guild_id() {
        helpers::post_to_spam_channel(response, ctx.serenity_context(), guild_id).await?;
    }

    Ok(())
}
