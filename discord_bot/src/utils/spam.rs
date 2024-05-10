use crate::prelude::Result;

use serenity::all::{Context, GuildId};

const SPAM_CHANNEL_NAME: &str = "botspam";

pub async fn post_to_spam_channel(
    text: impl Into<String>,
    ctx: &Context,
    guild_id: GuildId,
) -> Result<()> {
    let partial_guild = guild_id.to_partial_guild(ctx).await?;
    let Some(channel_id) = partial_guild.channel_id_from_name(ctx, SPAM_CHANNEL_NAME) else {
        println!("-> Could not find spam channel: \"{SPAM_CHANNEL_NAME}\"");
        return Ok(());
    };

    channel_id.say(ctx, text).await?;
    Ok(())
}
