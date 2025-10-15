#![allow(dead_code)]

use serenity::all::VoiceState;
use serenity::prelude::Context;

use crate::prelude::{CommandData, Error, Result, SongbirdError};

pub async fn handle_voice_state_update(
    ctx: &Context,
    old_state: &Option<VoiceState>,
    new_state: &VoiceState,
    data: &CommandData,
) -> Result<()> {
    // Ignore user joining a voice channel
    let Some(voice_state) = old_state else {
        return Ok(());
    };

    // Check if bot was the target of this event
    let member = match &new_state.member {
        Some(val) => val,
        None => {
            return Ok(());
        }
    };
    if member.user.id != ctx.cache.current_user().id {
        return Ok(());
    }

    // Handle if bot was moved to another channel or disconnected.
    if new_state.channel_id.is_some() {
        log::info!("Bot was moved to another voice channel.");
        return Ok(());
    }

    log::info!("Bot was disconnected from voice channel, clearing queue.");
    let manager = &data.songbird_manager;
    let guild_id = voice_state.guild_id.ok_or(Error::InvalidGuild)?;

    if manager.get(guild_id).is_some() {
        manager
            .remove(guild_id)
            .await
            .map_err(|err| Error::Songbird(Box::new(SongbirdError::SongbirdJoin(err))))?;
    }

    Ok(())
}
