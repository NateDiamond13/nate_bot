use database::pictures;
use poise::{CreateReply, command};
use serenity::all::CreateEmbed;

use crate::prelude::{Context, Result};

/// Base command for picture display, use subcommands.
#[command(
    slash_command,
    guild_only,
    category = "Pictures",
    rename = "pic",
    subcommands("pic_list", "pic_show", "pic_random", "pic_add", "pic_remove"),
    subcommand_required
)]
pub async fn pictures(_: Context<'_>) -> Result<()> {
    Ok(())
}

/// List all available pictures.
#[command(slash_command, category = "Pictures", rename = "list")]
pub async fn pic_list(ctx: Context<'_>) -> Result<()> {
    let guild_str = match ctx.guild_id() {
        Some(guild_id) => guild_id.to_string(),
        None => {
            ctx.say("Could not find current guild.").await?;
            return Ok(());
        }
    };

    let Some(pics) = pictures::get_all(&ctx.data().pool, &guild_str).await else {
        ctx.say("An error occurred while fetching pictures.")
            .await?;
        return Ok(());
    };
    if pics.is_empty() {
        ctx.say("No pictures found.").await?;
        return Ok(());
    }

    let picture_list: Vec<String> = pics
        .into_iter()
        .map(|pictures::Picture { name, .. }| name)
        .collect();

    let response = format!("Available pictures: {}", picture_list.join(", "));
    ctx.say(response).await?;
    Ok(())
}

/// Show picture with a given name, if it exists.
#[command(slash_command, category = "Pictures", rename = "show")]
pub async fn pic_show(
    ctx: Context<'_>,
    #[description = "Name of picture"] name: String,
) -> Result<()> {
    let guild_str = match ctx.guild_id() {
        Some(guild_id) => guild_id.to_string(),
        None => {
            ctx.say("Could not find current guild.").await?;
            return Ok(());
        }
    };

    let Some(pic) = pictures::get(&ctx.data().pool, &name, &guild_str).await else {
        ctx.say(format!("Could not find picture '{name}'.")).await?;
        return Ok(());
    };

    let embed = CreateEmbed::new().title(pic.name).image(pic.url);
    let response = CreateReply::default().embed(embed);
    ctx.send(response).await?;
    Ok(())
}

/// Show random picture.
#[command(slash_command, category = "Pictures", rename = "random")]
pub async fn pic_random(ctx: Context<'_>) -> Result<()> {
    let guild_str = match ctx.guild_id() {
        Some(guild_id) => guild_id.to_string(),
        None => {
            ctx.say("Could not find current guild.").await?;
            return Ok(());
        }
    };

    let Some(pic) = pictures::get_random(&ctx.data().pool, &guild_str, None).await else {
        ctx.say("Could not find random picture.").await?;
        return Ok(());
    };

    let embed = CreateEmbed::new().title(pic.name).image(pic.url);
    let response = CreateReply::default().embed(embed);
    ctx.send(response).await?;
    Ok(())
}

/// Add new picture with the given name, if it doesn't already exist.
#[command(slash_command, category = "Pictures", rename = "add")]
pub async fn pic_add(
    ctx: Context<'_>,
    name: String,
    url: String,
    #[flag] is_nsfw: bool,
) -> Result<()> {
    let guild_str = match ctx.guild_id() {
        Some(guild_id) => guild_id.to_string(),
        None => {
            ctx.say("Could not find current guild.").await?;
            return Ok(());
        }
    };

    let pool = &ctx.data().pool;
    if pictures::get(pool, &name, &guild_str).await.is_some() {
        ctx.say(format!("Picture '{name}' already exists.")).await?;
        return Ok(());
    }

    // Check formatting for name & url?

    let create_pic = pictures::CreatePicture {
        name: name.clone(),
        guild_id: guild_str,
        url,
        added_by_user: ctx.author().id.to_string(),
        is_nsfw,
    };
    match pictures::insert(pool, &create_pic).await {
        Ok(_) => {
            ctx.say(format!("Picture '{name}' successfully added."))
                .await?;
        }
        Err(_) => {
            ctx.say(format!("An error occurred while adding picture '{name}'."))
                .await?;
        }
    }
    Ok(())
}

/// Remove picture with the given name, if it exists.
#[command(slash_command, category = "Pictures", rename = "remove")]
pub async fn pic_remove(ctx: Context<'_>, name: String) -> Result<()> {
    let guild_str = match ctx.guild_id() {
        Some(guild_id) => guild_id.to_string(),
        None => {
            ctx.say("Could not find current guild.").await?;
            return Ok(());
        }
    };

    let pool = &ctx.data().pool;
    let Some(existing_pic) = pictures::get(pool, &name, &guild_str).await else {
        ctx.say(format!("Picture '{name}' not found.")).await?;
        return Ok(());
    };

    // Get the member
    let member = match ctx.author_member().await {
        Some(cow_member) => cow_member.into_owned(),
        None => {
            ctx.say("Guild member not found.").await?;
            return Ok(());
        }
    };

    // Check user permissions
    if member.user.id.to_string() != existing_pic.added_by_user
        && (member.permissions.is_none() || !member.permissions.unwrap().administrator())
    {
        ctx.say(format!(
            "Cannot remove '{name}'. Pictures can only be removed by admins or the user that added them."
        ))
        .await?;
        return Ok(());
    }

    // Remove the picture
    match pictures::remove(pool, &name, &guild_str).await {
        Ok(_) => {
            ctx.say(format!("Picture '{name}' successfully removed."))
                .await?;
        }
        Err(_) => {
            ctx.say(format!(
                "An error occurred while removing picture '{name}'."
            ))
            .await?;
        }
    }
    Ok(())
}
