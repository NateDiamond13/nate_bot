pub mod youtube;

use crate::prelude::{Context, Error, Result};

use serenity::all::ChannelId;
use songbird::Call;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn get_call(ctx: Context<'_>) -> Result<Option<Arc<Mutex<Call>>>> {
    // Get songbird manager and guild_id
    let manager = songbird::get(poise::Context::serenity_context(ctx))
        .await
        .ok_or(Error::InvalidGuild)?;
    let guild_id = ctx.guild_id().ok_or(Error::InvalidGuild)?;

    // Get call handler
    Ok(manager.get(guild_id))
}

pub async fn join_voice_channel(ctx: Context<'_>) -> Result<Arc<Mutex<Call>>> {
    // Get songbird manager
    let manager = songbird::get(poise::Context::serenity_context(ctx))
        .await
        .ok_or(Error::InvalidGuild)?;

    // Check if already in a call
    let guild_id = ctx.guild_id().ok_or(Error::InvalidGuild)?;
    if let Some(call) = manager.get(guild_id) {
        println!("Attempting to join, but already in a call.");
        return Ok(call);
    }

    // Get the id of the voice channel
    let user_id = ctx.author().id;
    let channel_id: ChannelId = ctx
        .cache()
        .guild(guild_id)
        .ok_or(Error::InvalidGuild)?
        .voice_states
        .get(&user_id)
        .and_then(|voice_state| voice_state.channel_id)
        .ok_or(Error::InvalidVoiceChannel)?;

    // Join the voice channel
    let success = manager.join(guild_id, channel_id).await?;
    Ok(success)
}

pub async fn leave_voice_channel(ctx: Context<'_>) -> Result<()> {
    let manager = songbird::get(poise::Context::serenity_context(ctx))
        .await
        .ok_or(Error::InvalidGuild)?;
    let guild_id = ctx.guild_id().ok_or(Error::InvalidGuild)?;

    if manager.get(guild_id).is_some() {
        manager.remove(guild_id).await?;
    }
    Ok(())
}
