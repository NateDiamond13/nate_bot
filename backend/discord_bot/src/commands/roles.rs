use poise::command;
use serenity::all::{Channel, EditRole, ReactionType};
use serenity::model::guild;
use serenity::small_fixed_array::FixedString;

use crate::events::{ROLE_CHANNEL, ROLE_REACTION};
use crate::prelude::{Context, Result};

/// Base command for role management, use subcommands. (Admin only)
#[command(
    slash_command,
    category = "Roles",
    subcommands("new_role", "post_role"),
    subcommand_required,
    guild_only,
    required_permissions = "ADMINISTRATOR"
)]
pub async fn roles(_: Context<'_>) -> Result<()> {
    Ok(())
}

/// Create a vote response for a new role in the server. (Admin only)
#[command(slash_command, category = "Roles", rename = "new")]
pub async fn new_role(
    ctx: Context<'_>,
    #[description = "Name of the new role to add"]
    #[min_length = 3]
    role: String,
) -> Result<()> {
    if !in_roles_channel(ctx).await {
        ctx.say(format!(
            "Command can only be used in #{ROLE_CHANNEL} channel"
        ))
        .await?;
        return Ok(());
    }

    if role.starts_with(['@', '<']) {
        ctx.say(format!("Invalid role name: '{role}'")).await?;
        return Ok(());
    }

    let Some(guild) = ctx.partial_guild().await else {
        ctx.say("Could not find guild").await?;
        return Ok(());
    };

    if guild.role_by_name(&role).is_some() {
        ctx.say(format!("Role '{role}' already exists")).await?;
        return Ok(());
    }

    let builder = EditRole::new().name(&role).mentionable(true);
    match guild.id.create_role(ctx.http(), builder).await {
        Ok(role) => {
            create_role_message(ctx, role.name).await?;
        }
        Err(why) => {
            ctx.say(format!("Failed to create role: {:?}", why)).await?;
        }
    };
    Ok(())
}

/// Create a vote response for an existing role in the server. (Admin only)
#[command(slash_command, category = "Roles", rename = "post")]
pub async fn post_role(
    ctx: Context<'_>,
    #[description = "Existing role to post"] role: guild::Role,
) -> Result<()> {
    if !in_roles_channel(ctx).await {
        ctx.say(format!(
            "Command can only be used in #{ROLE_CHANNEL} channel"
        ))
        .await?;
        return Ok(());
    }

    let Some(guild) = ctx.partial_guild().await else {
        ctx.say("Could not find guild").await?;
        return Ok(());
    };

    let role_name = role.name;
    match guild.role_by_name(&role_name) {
        Some(_) => {
            create_role_message(ctx, role_name).await?;
        }
        None => {
            ctx.say(format!(
                "Could not find role '{role_name}' in current guild"
            ))
            .await?;
        }
    };
    Ok(())
}

async fn create_role_message(ctx: Context<'_>, role_name: impl Into<String>) -> Result<()> {
    let reply_handle = ctx
        .say(format!(
            "React to this message with {} to be added to role '{}'",
            ROLE_REACTION,
            role_name.into()
        ))
        .await?;
    let reaction = ReactionType::Unicode(FixedString::from_static_trunc(ROLE_REACTION));
    reply_handle
        .message()
        .await?
        .react(ctx.http(), reaction)
        .await?;
    Ok(())
}

async fn in_roles_channel(ctx: Context<'_>) -> bool {
    match ctx.channel().await {
        Some(Channel::Guild(channel)) => channel.base.name == ROLE_CHANNEL,
        Some(Channel::GuildThread(thread)) => thread.base.name == ROLE_CHANNEL,
        Some(_) => false,
        None => false,
    }
}
