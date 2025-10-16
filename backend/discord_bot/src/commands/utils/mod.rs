mod sounds;

use std::sync::Arc;

use serenity::all::GuildId;
use serenity::async_trait;
use songbird::{Call, Event, EventContext, EventHandler, Songbird, TrackEvent};
pub use sounds::get_sound_details;
use tokio::sync::Mutex;

use crate::prelude::{Context, Error, Result, SongbirdError};

struct SoundEndNotifier {
    manager: Arc<Songbird>,
    guild_id: GuildId,
}

#[async_trait]
impl EventHandler for SoundEndNotifier {
    async fn act(&self, _ctx: &EventContext<'_>) -> Option<Event> {
        let call = self.manager.get(self.guild_id)?;
        if call.lock().await.queue().is_empty()
            && let Err(err) = self.manager.remove(self.guild_id).await
        {
            log::error!("Error while sound ends: {err}");
        }

        None
    }
}

pub async fn get_call(ctx: Context<'_>) -> Result<Option<Arc<Mutex<Call>>>> {
    // Get guild_id and call handler
    let guild_id = ctx.guild_id().ok_or(Error::InvalidGuild)?;
    Ok(ctx.data().songbird_manager.get(guild_id))
}

pub async fn join_voice_channel(ctx: Context<'_>) -> Result<()> {
    // Check if already in a call
    let manager = &ctx.data().songbird_manager;
    let guild_id = ctx.guild_id().ok_or(Error::InvalidGuild)?;
    if manager.get(guild_id).is_some() {
        log::error!("Attempting to join, but already in a call.");
        return Ok(());
    }

    // Get the id of the voice channel
    let user_id = ctx.author().id;
    let channel_id = ctx
        .guild()
        .ok_or(Error::InvalidGuild)?
        .voice_states
        .get(&user_id)
        .and_then(|voice_state| voice_state.channel_id)
        .ok_or(Error::InvalidVoiceChannel)?;

    // Join the voice channel
    let handle_lock = manager
        .join(guild_id, channel_id)
        .await
        .map_err(|err| Error::Songbird(Box::new(SongbirdError::SongbirdJoin(err))))?;

    // Add event handler to leave voice channel when queue is empty
    let mut handle = handle_lock.lock().await;
    handle.add_global_event(
        Event::Track(TrackEvent::End),
        SoundEndNotifier {
            manager: manager.clone(),
            guild_id,
        },
    );

    Ok(())
}

pub async fn leave_voice_channel(ctx: Context<'_>) -> Result<()> {
    let manager = &ctx.data().songbird_manager;
    let guild_id = ctx.guild_id().ok_or(Error::InvalidGuild)?;

    if manager.get(guild_id).is_some() {
        manager
            .remove(guild_id)
            .await
            .map_err(|err| Error::Songbird(Box::new(SongbirdError::SongbirdJoin(err))))?;
    }
    Ok(())
}
