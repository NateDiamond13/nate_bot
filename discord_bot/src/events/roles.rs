use crate::prelude::{Error, Result};

use regex::Regex;
use serenity::all::{Channel, Reaction, ReactionType};
use serenity::prelude::Context;

pub const ROLE_CHANNEL: &str = "roles";
pub const ROLE_REACTION: char = '👍';

pub async fn handle_reaction_add(ctx: &Context, event: &Reaction) -> Result<()> {
    if let Err(error) = toggle_user_role(ctx, event, true).await {
        println!("Error in role reaction add: {error}");
    }
    Ok(())
}

pub async fn handle_reaction_remove(ctx: &Context, event: &Reaction) -> Result<()> {
    if let Err(error) = toggle_user_role(ctx, event, false).await {
        println!("Error in role reaction remove: {error}");
    }
    Ok(())
}

async fn toggle_user_role(
    ctx: &Context,
    reaction: &Reaction,
    add_role: bool,
) -> core::result::Result<(), Box<dyn std::error::Error>> {
    if reaction.emoji != ReactionType::Unicode(ROLE_REACTION.to_string()) {
        return Ok(());
    }

    // Check if user is a bot
    let user = reaction.user(ctx).await?;
    if user.bot {
        return Ok(());
    }

    // Check if message is from this bot
    let message = reaction.message(ctx).await?;
    if !message.is_own(ctx) {
        return Ok(());
    }

    // Check if reaction was in the correct channel
    let Channel::Guild(channel) = reaction.channel(ctx).await? else {
        return Ok(());
    };
    if channel.name() != ROLE_CHANNEL {
        return Ok(());
    }

    // Parse message for role name
    let role_name = parse_for_role(message.content)?;

    // Check if role exists in guild
    let guild_id = channel.guild_id;
    let guild = guild_id.to_partial_guild(ctx).await?;
    let role = guild
        .role_by_name(&role_name)
        .ok_or_else(|| Error::ToggleRoleFailure)?;

    // Check if user already has role
    let has_role = user.has_role(ctx, guild_id, role).await?;
    let member = guild.member(ctx, user.id).await?;

    // Toggle role for user
    if add_role && !has_role {
        member.add_role(ctx, role).await?;
        println!(
            "Added user '{}' to role '{}' in guild '{}'",
            user.name, role.name, guild.name
        );
    } else if !add_role && has_role {
        member.remove_role(ctx, role).await?;
        println!(
            "Removed user '{}' from role '{}' in guild '{}'",
            user.name, role.name, guild.name
        );
    }
    Ok(())
}

fn parse_for_role(message_str: impl Into<String>) -> Result<String> {
    let msg = message_str.into();
    let re = Regex::new(r"^React(?:.*)\'(.{3,})'$")?;
    let (_, [role_name]) = re
        .captures(&msg)
        .ok_or_else(|| Error::ToggleRoleFailure)?
        .extract();
    Ok(role_name.to_string())
}

#[cfg(test)]
mod tests {
    use crate::events::roles::parse_for_role;

    #[test]
    fn parse_good_role() {
        let s1 = String::from("React to this message with 👍 to be added to role 'good'");
        assert_eq!(parse_for_role(s1).unwrap(), "good".to_string());

        let s2 = String::from("React to this message with 👍 to be added to role 'foo'");
        assert_eq!(parse_for_role(s2).unwrap(), "foo".to_string());

        let s3 = String::from("React to role 'shama llama'");
        assert_eq!(parse_for_role(s3).unwrap(), "shama llama".to_string());
    }

    #[test]
    fn parse_bad_role() {
        let s1 = String::from("React to this message with 👍 to be added to role '@'");
        assert!(parse_for_role(s1).is_err());

        let s2 = String::from("Please react to this message with 👍 to be added to role 'good'");
        assert!(parse_for_role(s2).is_err());

        let s3 = String::from("React to this message with 👍 to be added to role ''");
        assert!(parse_for_role(s3).is_err());

        let s4 = String::from("React to this message with 👍 to be added to role 'no'");
        assert!(parse_for_role(s4).is_err());

        let s5 = String::from("");
        assert!(parse_for_role(s5).is_err());
    }
}
