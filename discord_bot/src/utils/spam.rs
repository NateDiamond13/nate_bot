use crate::prelude::Result;

use serenity::all::{ChannelId, Context, GuildId};

const SPAM_CHANNEL_NAME: &str = "botspam";

pub async fn post_to_spam_channel(
    text: impl Into<String>,
    ctx: &Context,
    guild_id: GuildId,
) -> Result<()> {
    let Some(channel_id) = channel_id_from_name(ctx, guild_id, SPAM_CHANNEL_NAME).await else {
        println!("-> Could not find spam channel: \"{SPAM_CHANNEL_NAME}\"");
        return Ok(());
    };

    channel_id.say(ctx, text).await?;
    Ok(())
}

async fn channel_id_from_name(ctx: &Context, guild_id: GuildId, name: &str) -> Option<ChannelId> {
    let channels = guild_id.channels(ctx).await.ok()?;
    channels.values().find(|c| c.name == name).map(|c| c.id)
}
