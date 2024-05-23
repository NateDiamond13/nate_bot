mod audit;
mod lottery;
mod music;
mod reactions;
mod ready;
mod roles;
pub use roles::{ROLE_CHANNEL, ROLE_REACTION};

use crate::prelude::{CommandData, Error, Result};

use poise::FrameworkContext;
use serenity::all::FullEvent;
use serenity::prelude::Context;

pub async fn event_handler(
    ctx: &Context,
    event: &FullEvent,
    _framework: FrameworkContext<'_, CommandData, Error>,
    data: &CommandData,
) -> Result<()> {
    match event {
        FullEvent::Ready { data_about_bot } => {
            ready::handle_ready(ctx, data_about_bot)?;
        }
        FullEvent::ReactionAdd { add_reaction } => {
            roles::handle_reaction_add(ctx, add_reaction).await?;
        }
        FullEvent::ReactionRemove { removed_reaction } => {
            roles::handle_reaction_remove(ctx, removed_reaction).await?;
        }
        FullEvent::VoiceStateUpdate { old, new } => {
            audit::handle_voice_state_update(ctx, old, new).await?;
            music::handle_voice_state_update(ctx, old, new).await?;
        }
        FullEvent::Message { new_message } => {
            lottery::handle_message(ctx, new_message, data).await?;
            reactions::handle_message(ctx, new_message, data).await?;
        }
        _ => {}
    }
    Ok(())
}
