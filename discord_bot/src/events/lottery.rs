use crate::db::pictures;
use crate::prelude::{CommandData, Result};

use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use serenity::all::{CreateEmbed, CreateMessage, Mentionable, Message};
use serenity::prelude::Context;

pub async fn handle_message(ctx: &Context, message: &Message, data: &CommandData) -> Result<()> {
    // Check if author is a bot
    if message.author.bot {
        return Ok(());
    }

    // Check if message passes lottery odds
    if StdRng::from_entropy().gen_range(0..data.env.lottery_odds) != 0 {
        return Ok(());
    }

    // Get a random picture
    let Some(picture) = pictures::get_random(&data.pool, Some(false)).await else {
        return Ok(());
    };

    // Send congratulatory picture embed
    let response = CreateMessage::default()
        .embed(
            CreateEmbed::new()
                .title("Congratulations!")
                .image(picture.url),
        )
        .content(format!(
            "{} won the lottery with 1/{} odds!",
            message.author.mention(),
            data.env.lottery_odds
        ));
    message.channel_id.send_message(ctx, response).await?;
    Ok(())
}
