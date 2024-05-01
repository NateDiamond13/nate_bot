use crate::prelude::{Error, Result};
use crate::utils;

use serenity::all::{Channel, GuildChannel, VoiceState};
use serenity::model::guild::audit_log::{Action, MemberAction};
use serenity::prelude::Context;

pub async fn handle_voice_state_update(
    ctx: &Context,
    old_state: &Option<VoiceState>,
    new_state: &VoiceState,
) -> Result<()> {
    // Ignore user joining a voice channel
    let old_state = match old_state {
        Some(state) => state,
        None => {
            return Ok(());
        }
    };

    // Ignore events from bots
    let member = match &new_state.member {
        Some(val) => val,
        None => {
            return Ok(());
        }
    };
    if member.user.bot {
        return Ok(());
    }

    // Get the latest audit log entry with the given action
    let action_type = match new_state.channel_id {
        Some(_) => Action::Member(MemberAction::MemberMove),
        None => Action::Member(MemberAction::MemberDisconnect),
    };
    let audit_logs = member
        .guild_id
        .audit_logs(&ctx.http, Some(action_type), None, None, Some(1))
        .await?;

    // Check if entry was created in the last second
    let entry = match audit_logs.entries.first() {
        Some(entry) => entry,
        None => {
            return Ok(());
        }
    };
    let entry_time = entry.id.created_at().time();
    let current_time = chrono::Utc::now().time();
    if current_time.signed_duration_since(entry_time).num_seconds() >= 1 {
        return Ok(());
    }

    // Get user and channel info to print out
    let user = entry.user_id.to_user(&ctx.http).await?;
    let old_channel_name = get_channel_name(old_state, ctx).await?;

    match action_type {
        Action::Member(MemberAction::MemberMove) => {
            let new_channel_name = get_channel_name(new_state, ctx).await?;
            let response = format!(
                "User '{}' moved '{}' from channel '{}' to '{}'",
                user.name, member.user.name, old_channel_name, new_channel_name
            );
            println!("{response}");
            utils::post_to_spam_channel(response, ctx, member.guild_id).await?;
        }
        Action::Member(MemberAction::MemberDisconnect) => {
            let response = format!(
                "User '{}' disconnected '{}' from channel '{}'",
                user.name, member.user.name, old_channel_name
            );
            println!("{response}");
            utils::post_to_spam_channel(response, ctx, member.guild_id).await?;
        }
        _ => {}
    }
    Ok(())
}

async fn get_channel_name(voice_state: &VoiceState, context: &Context) -> Result<String> {
    let channel_id = match voice_state.channel_id {
        Some(id) => id,
        None => {
            return Err(Error::InvalidVoiceChannel);
        }
    };
    match channel_id.to_channel(&context.http).await {
        Ok(Channel::Guild(GuildChannel { name, .. })) => Ok(name),
        _ => Err(Error::InvalidVoiceChannel),
    }
}
