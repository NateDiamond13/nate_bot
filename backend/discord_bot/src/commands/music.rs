use crate::commands::utils;
use crate::prelude::{Context, Error, Result};

use poise::{CreateReply, ReplyHandle, command};

/// Queue sound from given url or search string
#[command(slash_command, guild_only, category = "Music")]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Url of video or string of text to search for"] url_or_search: String,
    #[flag]
    #[rename = "f"]
    #[description = "(Force): Set this flag to clear the queue before playing the sound."]
    force: bool,
) -> Result<()> {
    let reply_msg = ctx.say("Attempting to queue sound...").await?;
    println!(
        "User '{}' adding sound to queue: '{}'",
        ctx.author().name,
        url_or_search
    );

    // Attempt download from url or search string
    let sound_details = match utils::get_sound_details(&url_or_search, &ctx.data()).await {
        Ok(Some(details)) => details,
        Ok(None) => {
            let response = format!("Invalid sound source: {url_or_search}.");
            println!("{response}");
            update_reply(ctx, reply_msg, response).await?;
            return Ok(());
        }
        Err(err) => {
            let response =
                format!("Error occurred while getting sound details for \"{url_or_search}\".");
            println!("{response}");
            update_reply(ctx, reply_msg, response).await?;
            return Err(err);
        }
    };

    if sound_details.num_seconds > 7200 {
        let response = "Cannot queue sounds longer than 2 hours.";
        println!("{response}");
        update_reply(ctx, reply_msg, response).await?;
        return Ok(());
    }

    // Join the voice channel
    match utils::join_voice_channel(ctx).await {
        Ok(()) => {}
        Err(Error::InvalidVoiceChannel) => {
            let response = "User is not currently in a voice channel.";
            println!("{response}");
            update_reply(ctx, reply_msg, response).await?;
            return Ok(());
        }
        Err(err) => {
            update_reply(ctx, reply_msg, "Bot could not join voice channel.").await?;
            return Err(err);
        }
    };

    // Get call handler
    let call = match utils::get_call(ctx).await {
        Ok(Some(call)) => call,
        Ok(None) => {
            let response = "Could not find current voice channel.";
            println!("{response}");
            update_reply(ctx, reply_msg, response).await?;
            return Ok(());
        }
        Err(err) => {
            update_reply(ctx, reply_msg, "Bot could not join voice channel.").await?;
            return Err(err);
        }
    };
    let mut handler = call.lock().await;
    if force {
        handler.queue().stop();
    }

    let max_sounds = ctx.data().env.queue_max_sounds;
    if handler.queue().len() < max_sounds {
        handler.enqueue_input(sound_details.input).await;
    } else {
        let response = format!("Maximum sounds ({max_sounds}) already in queue.");
        println!("{response}");
        update_reply(ctx, reply_msg, response).await?;
        return Ok(());
    }

    // Get current length of queue
    let response = match handler.queue().len() {
        n if n > 1 => format!("Added to queue ({}): {}", n, sound_details.source_url),
        _ => format!("Playing sound: {}", sound_details.source_url),
    };

    update_reply(ctx, reply_msg, response).await?;
    Ok(())
}

/// Skip the currently playing sound
#[command(slash_command, guild_only, category = "Music")]
pub async fn skip(ctx: Context<'_>) -> Result<()> {
    let Some(call) = utils::get_call(ctx).await? else {
        ctx.say("Bot is not currently in a voice channel.").await?;
        return Ok(());
    };

    let queue_len;
    // Scope the handler so we drop the lock after clearing
    {
        let handler = call.lock().await;
        let queue = handler.queue();

        match queue.skip() {
            Ok(_) => {
                queue_len = queue.len() - 1;
                if queue_len == 0 {
                    ctx.say("Skipping sound, queue empty.").await?;
                } else {
                    ctx.say(format!("Skipping sound, {queue_len} in queue."))
                        .await?;
                }
            }
            Err(e) => {
                println!("{e}");
                ctx.say("Error occurred while trying to skip sound.")
                    .await?;
                return Ok(());
            }
        }
    }

    // Leave the voice channel if the queue is empty
    if queue_len == 0 {
        utils::leave_voice_channel(ctx).await?;
    }
    Ok(())
}

/// Stop playback and clear the queue
#[command(slash_command, guild_only, category = "Music")]
pub async fn stop(ctx: Context<'_>) -> Result<()> {
    let Some(call) = utils::get_call(ctx).await? else {
        ctx.say("Bot is not currently in a voice channel.").await?;
        return Ok(());
    };

    // Scope the handler so we drop the lock after clearing
    {
        let handler = call.lock().await;
        let queue = handler.queue();
        ctx.say(format!(
            "Playback stopped, clearing {} sound{} in queue.",
            queue.len(),
            if queue.len() > 1 { "s" } else { "" }
        ))
        .await?;
        queue.stop();
    }

    // Leave the voice channel
    utils::leave_voice_channel(ctx).await?;
    Ok(())
}

async fn update_reply<'a>(
    ctx: Context<'_>,
    reply: ReplyHandle<'a>,
    content: impl Into<String>,
) -> Result<()> {
    reply
        .edit(ctx, CreateReply::default().content(content.into()))
        .await?;
    Ok(())
}
