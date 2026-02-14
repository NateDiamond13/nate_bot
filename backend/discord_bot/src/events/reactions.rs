use std::cmp;

use rand::rngs::{StdRng, SysRng};
use rand::seq::{IteratorRandom, SliceRandom};
use rand::{SeedableRng, random_range};
use serenity::all::{CacheHttp, Message};
use serenity::prelude::Context;

use crate::prelude::{CommandData, Result};

const REACTION_COUNT_MIN: usize = 5;
const REACTION_COUNT_MAX: usize = 20;

pub async fn handle_message(ctx: &Context, message: &Message, data: &CommandData) -> Result<()> {
    // Check if message author is a target
    let user_id: u64 = u64::from(message.author.id);
    if !data.env.reaction_target_ids.contains(&user_id) {
        return Ok(());
    }

    // Check if message passes reaction odds
    if random_range(0..data.env.reaction_target_odds) != 0 {
        return Ok(());
    }

    // Get all the custom emojis from the current guild
    let Some(guild_id) = message.guild_id else {
        return Ok(());
    };
    let emojis = guild_id.emojis(ctx.http()).await?;

    // Choose how many emojis to react with
    let min_count = cmp::min(emojis.len(), REACTION_COUNT_MIN);
    let max_count = cmp::min(emojis.len(), REACTION_COUNT_MAX);
    let emoji_count = random_range(min_count..=max_count);
    let mut rng = StdRng::try_from_rng(&mut SysRng)?;
    let mut choices = emojis.iter().sample(&mut rng, emoji_count);
    choices.shuffle(&mut rng);

    log::info!(
        "Reacting to a message from '{}' with {} emoji(s) - Odds: 1/{}",
        message.author.name,
        emoji_count,
        data.env.reaction_target_odds
    );

    // React to the message with the chosen emojis
    for choice in choices {
        message.react(ctx.http(), choice.clone()).await?;
    }

    Ok(())
}
