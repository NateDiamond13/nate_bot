use crate::prelude::{Error, Result};

use serenity::all::VoiceState;
use serenity::prelude::Context;

pub async fn handle_voice_state_update(
    ctx: &Context,
    old_state: &Option<VoiceState>,
    new_state: &VoiceState,
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
        println!("Bot was moved to another voice channel.");
        return Ok(());
    }

    println!("Bot was disconnected from voice channel, clearing queue.");
    let manager = songbird::get(ctx).await.ok_or(Error::InvalidGuild)?;
    let guild_id = voice_state.guild_id.ok_or(Error::InvalidGuild)?;

    if manager.get(guild_id).is_some() {
        manager.remove(guild_id).await?;
    }

    Ok(())
}
