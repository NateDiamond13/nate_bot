use serenity::all::{CacheHttp, Context, GenericChannelId, GuildId};

use crate::prelude::Result;

const SPAM_CHANNEL_NAME: &str = "botspam";

pub async fn post_to_spam_channel(
    text: impl Into<String>,
    ctx: &Context,
    guild_id: GuildId,
) -> Result<()> {
    let Some(channel_id) = channel_id_from_name(ctx, guild_id, SPAM_CHANNEL_NAME).await else {
        log::error!("-> Could not find spam channel: \"{SPAM_CHANNEL_NAME}\"");
        return Ok(());
    };

    channel_id.say(ctx.http(), text.into()).await?;
    Ok(())
}

async fn channel_id_from_name(
    ctx: &Context,
    guild_id: GuildId,
    name: &str,
) -> Option<GenericChannelId> {
    let channels = guild_id.channels(ctx.http()).await.ok()?;
    channels
        .iter()
        .find(|c| c.base.name == name)
        .map(|c| c.id.widen())
}
