#![allow(dead_code)]

use lavalink_rs::model::GuildId;
use lavalink_rs::model::player::ConnectionInfo;
use lavalink_rs::prelude::{SearchEngines, TrackInQueue, TrackLoadData};
use poise::command;
use poise::serenity_prelude::Mentionable;

use crate::prelude::{Context, Error, LavalinkError, Result, SongbirdError};

async fn join_voice_channel(ctx: &Context<'_>) -> Result<()> {
    // Check if already in a call
    let data = &ctx.data();
    let manager = &data.songbird_manager;
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

    let handler = manager.join_gateway(guild_id, channel_id).await;
    match handler {
        Ok((connection_info, _)) => {
            let lava_connection = ConnectionInfo {
                endpoint: connection_info.endpoint,
                token: connection_info.token,
                session_id: connection_info.session_id,
            };

            {
                let lavalink_client = data.lavalink_client.try_lock()?;
                lavalink_client
                    .create_player_context(guild_id.get(), lava_connection)
                    .await
                    .map_err(|err| Error::Lavalink(Box::new(LavalinkError::Lavalink(err))))?;
            }

            ctx.say(format!("Joined {}", channel_id.mention())).await?;

            Ok(())
        }
        Err(err) => {
            ctx.say(format!("Error joining the channel: {}", err))
                .await?;
            Err(Error::Songbird(Box::new(SongbirdError::SongbirdJoin(err))))
        }
    }
}

async fn leave_voice_channel(ctx: Context<'_>) -> Result<()> {
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

async fn get_lavalink_guild_id(ctx: &Context<'_>) -> Result<GuildId> {
    let guild_id: GuildId = ctx.guild_id().ok_or(Error::InvalidGuild)?.get().into();
    Ok(guild_id)
}

/// Play a song in the voice channel you are connected in.
#[command(slash_command, guild_only, category = "Music")]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Search term or URL"]
    #[rest]
    term: Option<String>,
) -> Result<()> {
    // Join the voice channel
    match join_voice_channel(&ctx).await {
        Ok(()) => {}
        Err(Error::InvalidVoiceChannel) => {
            let response = "User is not currently in a voice channel.";
            log::error!("{response}");
            ctx.say(response).await?;
            return Ok(());
        }
        Err(err) => {
            ctx.say("Bot could not join voice channel.").await?;
            return Err(err);
        }
    };

    let guild_id = get_lavalink_guild_id(&ctx).await?;
    let data = ctx.data();
    let lavalink_client = data.lavalink_client.try_lock()?;

    let Some(player) = lavalink_client.get_player_context(guild_id) else {
        ctx.say("Join the bot to a voice channel first.").await?;
        return Ok(());
    };

    let query = if let Some(term) = term {
        if term.starts_with("http") {
            term
        } else {
            SearchEngines::YouTube
                .to_query(&term)
                .map_err(|err| Error::Lavalink(Box::new(LavalinkError::Lavalink(err))))?
        }
    } else {
        if let Ok(player_data) = player.get_player().await {
            let queue = player.get_queue();

            if player_data.track.is_none() && queue.get_track(0).await.is_ok_and(|x| x.is_some()) {
                player
                    .skip()
                    .map_err(|err| Error::Lavalink(Box::new(LavalinkError::Lavalink(err))))?;
            } else {
                ctx.say("The queue is empty.").await?;
            }
        }

        return Ok(());
    };

    let loaded_tracks = lavalink_client
        .load_tracks(guild_id, &query)
        .await
        .map_err(|err| Error::Lavalink(Box::new(LavalinkError::Lavalink(err))))?;

    let mut playlist_info = None;

    let mut tracks: Vec<TrackInQueue> = match loaded_tracks.data {
        Some(TrackLoadData::Track(x)) => vec![x.into()],
        Some(TrackLoadData::Search(x)) => vec![x[0].clone().into()],
        Some(TrackLoadData::Playlist(x)) => {
            playlist_info = Some(x.info);
            x.tracks.iter().map(|x| x.clone().into()).collect()
        }

        _ => {
            ctx.say(format!("{:?}", loaded_tracks)).await?;
            return Ok(());
        }
    };

    if let Some(info) = playlist_info {
        ctx.say(format!("Added playlist to queue: {}", info.name,))
            .await?;
    } else {
        let track = &tracks[0].track;

        if let Some(uri) = &track.info.uri {
            ctx.say(format!(
                "Added to queue: [{} - {}](<{}>)",
                track.info.author, track.info.title, uri
            ))
            .await?;
        } else {
            ctx.say(format!(
                "Added to queue: {} - {}",
                track.info.author, track.info.title
            ))
            .await?;
        }
    }

    for i in &mut tracks {
        i.track.user_data = Some(serde_json::json!({"requester_id": ctx.author().id.get()}));
    }

    let queue = player.get_queue();
    queue
        .append(tracks.into())
        .map_err(|err| Error::Lavalink(Box::new(LavalinkError::Lavalink(err))))?;

    if let Ok(player_data) = player.get_player().await
        && player_data.track.is_none()
        && queue.get_track(0).await.is_ok_and(|x| x.is_some())
    {
        player
            .skip()
            .map_err(|err| Error::Lavalink(Box::new(LavalinkError::Lavalink(err))))?;
    }

    Ok(())
}

/// Stop playback and leave the channel
#[command(slash_command, guild_only, category = "Music")]
pub async fn stop(ctx: Context<'_>) -> Result<()> {
    let guild_id = ctx.guild_id().ok_or(Error::InvalidGuild)?;

    let data = ctx.data();
    let lavalink_client = data.lavalink_client.try_lock()?;
    lavalink_client
        .delete_player(guild_id.get())
        .await
        .map_err(|err| Error::Lavalink(Box::new(LavalinkError::Lavalink(err))))?;

    leave_voice_channel(ctx).await?;

    ctx.say("Left voice channel.").await?;

    Ok(())
}
