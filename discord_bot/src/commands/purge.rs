use crate::{
    prelude::{Context, Result},
    utils,
};

use poise::{command, PrefixContext};
use serenity::all::{GetMessages, Message};

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
    #[flag]
    #[description = "(Silent): Set this flag to suppress the output message"]
    s: bool,
    #[flag]
    #[description = "(Delete): Set this flag to delete the original message (prefix command only)"]
    d: bool,
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
        message_filter = GetMessages::new().limit(PURGE_LIMIT).before(msg);
    } else {
        message_filter = GetMessages::new().limit(PURGE_LIMIT);
    }

    let messages_to_delete = channel.messages(ctx, message_filter).await?;
    let message_ids = messages_to_delete
        .iter()
        .filter(|&msg| msg.content.to_lowercase().contains(&text_lower))
        .collect::<Vec<&Message>>();
    channel.delete_messages(ctx, &message_ids).await?;

    // Handle delete (d) flag
    if d {
        if let Some(msg) = current_message {
            msg.delete(ctx).await?;
        }
    }

    // Handle silent (s) flag
    let response = format!(
        "User {} purged {} message(s) containing \"{}\" from channel '{}'",
        ctx.author().name,
        message_ids.len(),
        text,
        channel.name
    );
    println!("{response}");
    if !s {
        if let Some(guild_id) = ctx.guild_id() {
            utils::post_to_spam_channel(response, ctx.serenity_context(), guild_id).await?;
        }
    }

    Ok(())
}
