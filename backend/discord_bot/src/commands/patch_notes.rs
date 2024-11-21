use std::fmt;

use crate::prelude::{Context, Result};

use database::{
    patch_notes::{self},
    patch_notes_subscriptions::{self, CreatePatchNotesSub},
};

use poise::{command, ChoiceParameter, CreateReply};
use serenity::all::{CreateWebhook, Webhook};

#[derive(ChoiceParameter, Debug)]
pub enum PatchGame {
    #[name = "deadlock"]
    Deadlock,
}

impl fmt::Display for PatchGame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PatchGame::Deadlock => write!(f, "Deadlock"),
        }
    }
}

/// Base command for patch notes, use subcommands.
#[command(
    slash_command,
    category = "Patch Notes",
    rename = "patch",
    subcommands("patch_latest", "patch_sub", "patch_unsub"),
    subcommand_required
)]
pub async fn patch_notes(_: Context<'_>) -> Result<()> {
    Ok(())
}

/// Get the latest patch notes for a game.
#[command(slash_command, category = "Patch Notes", rename = "latest")]
pub async fn patch_latest(
    ctx: Context<'_>,
    #[description = "Target game"] game: PatchGame,
) -> Result<()> {
    let Some(latest_patch) = patch_notes::get_latest(&ctx.data().pool, game.to_string()).await
    else {
        ctx.say(format!("Could not find patch notes for game: {game}"))
            .await?;
        return Ok(());
    };

    let embed = webhooks::patch_notes::create_patch_embed(&latest_patch);
    let response = CreateReply::default().embed(embed);
    ctx.send(response).await?;
    Ok(())
}

/// Subscribe this channel to receive patch notifications for a game. (Admin only)
#[command(
    slash_command,
    guild_only,
    category = "Patch Notes",
    rename = "sub",
    required_permissions = "ADMINISTRATOR"
)]
pub async fn patch_sub(
    ctx: Context<'_>,
    #[description = "Target game"] game: PatchGame,
) -> Result<()> {
    let Some(guild_channel) = ctx.guild_channel().await else {
        ctx.say("Could not find current channel").await?;
        return Ok(());
    };

    let guild_id = guild_channel.guild_id.to_string();
    let channel_id = guild_channel.id.to_string();

    // Check if sub already exists
    if patch_notes_subscriptions::get(&ctx.data().pool, game.to_string(), &guild_id, &channel_id)
        .await
        .is_some()
    {
        ctx.say(format!(
            "This channel is already subscribed for: **{game}**"
        ))
        .await?;
        return Ok(());
    }

    // Create new webhook
    let current_user = ctx.cache().current_user().to_owned();
    let builder = CreateWebhook::new(format!("{} Patch Notes - {}", current_user.name, game));
    let hook = guild_channel.create_webhook(ctx.http(), builder).await?;
    let Some(token) = hook.token else {
        let reason = "Unable to create patch notes webhook";
        hook.delete(ctx.http(), Some(reason)).await?;
        ctx.say(reason).await?;
        return Ok(());
    };

    // Add sub to database
    let create_sub = CreatePatchNotesSub {
        target_game: game.to_string(),
        guild_id,
        channel_id,
        webhook_id: hook.id.into(),
        webhook_token: token.expose_secret().to_string(),
    };
    patch_notes_subscriptions::insert(&ctx.data().pool, &create_sub).await?;

    ctx.say(format!(
        "This channel will now receive patch notifications for: **{game}**"
    ))
    .await?;

    Ok(())
}

/// Unsubscribe this channel from receiving patch notifications for a game. (Admin only)
#[command(
    slash_command,
    guild_only,
    category = "Patch Notes",
    rename = "unsub",
    required_permissions = "ADMINISTRATOR"
)]
pub async fn patch_unsub(
    ctx: Context<'_>,
    #[description = "Target game"] game: PatchGame,
) -> Result<()> {
    let Some(guild_channel) = ctx.guild_channel().await else {
        ctx.say("Could not find current channel").await?;
        return Ok(());
    };

    let guild_id = guild_channel.guild_id.to_string();
    let channel_id = guild_channel.id.to_string();

    // Check if sub exists
    let Some(Ok(current_sub)) =
        patch_notes_subscriptions::get(&ctx.data().pool, game.to_string(), &guild_id, &channel_id)
            .await
    else {
        ctx.say(format!("This channel is not subscribed for: **{game}**"))
            .await?;
        return Ok(());
    };

    // Remove the webhook
    if let Ok(hook) = Webhook::from_id_with_token(
        ctx.http(),
        current_sub.webhook_id.into(),
        &current_sub.webhook_token,
    )
    .await
    {
        hook.delete(ctx.http(), None).await?;
    }

    // Remove the sub
    patch_notes_subscriptions::remove(
        &ctx.data().pool,
        &current_sub.target_game,
        &current_sub.guild_id,
        &current_sub.channel_id,
    )
    .await?;
    ctx.say(format!(
        "This channel will no longer receive notifications for: **{game}**"
    ))
    .await?;
    Ok(())
}
