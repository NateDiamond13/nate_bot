mod videos;
pub use videos::get_video_details;

use crate::prelude::{Context, Error, Result};

use serenity::{
    all::{ChannelId, GuildId},
    async_trait,
};
use songbird::{Call, Event, EventContext, EventHandler, Songbird, TrackEvent};
use std::sync::Arc;
use tokio::sync::Mutex;

struct SoundEndNotifier {
    manager: Arc<Songbird>,
    guild_id: GuildId,
}

#[async_trait]
impl EventHandler for SoundEndNotifier {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        let call = self.manager.get(self.guild_id)?;
        if call.lock().await.queue().is_empty() {
            if let Err(e) = self.manager.remove(self.guild_id).await {
                println!("Error while sound ends: {e}");
            }
        }

        None
    }
}

pub async fn get_call(ctx: Context<'_>) -> Result<Option<Arc<Mutex<Call>>>> {
    // Get songbird manager and guild_id
    let manager = songbird::get(poise::Context::serenity_context(ctx))
        .await
        .ok_or(Error::InvalidGuild)?;
    let guild_id = ctx.guild_id().ok_or(Error::InvalidGuild)?;

    // Get call handler
    Ok(manager.get(guild_id))
}

pub async fn join_voice_channel(ctx: Context<'_>) -> Result<()> {
    // Get songbird manager
    let manager = songbird::get(poise::Context::serenity_context(ctx))
        .await
        .ok_or(Error::InvalidGuild)?;

    // Check if already in a call
    let guild_id = ctx.guild_id().ok_or(Error::InvalidGuild)?;
    if manager.get(guild_id).is_some() {
        println!("Attempting to join, but already in a call.");
        return Ok(());
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
    let handle_lock = manager.join(guild_id, channel_id).await?;

    // Add event handler to leave voice channel when queue is empty
    let mut handle = handle_lock.lock().await;
    handle.add_global_event(
        Event::Track(TrackEvent::End),
        SoundEndNotifier { manager, guild_id },
    );

    Ok(())
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
