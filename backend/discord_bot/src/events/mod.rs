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

pub async fn event_handler(
    framework_context: FrameworkContext<'_, CommandData, Error>,
    event: &FullEvent,
) -> Result<()> {
    let ctx = framework_context.serenity_context;
    let data = framework_context.user_data();
    match event {
        FullEvent::Ready { data_about_bot } => {
            ready::handle_ready(&framework_context, data_about_bot).await?;
        }
        FullEvent::ReactionAdd { add_reaction } => {
            roles::handle_reaction_add(ctx, add_reaction).await?;
        }
        FullEvent::ReactionRemove { removed_reaction } => {
            roles::handle_reaction_remove(ctx, removed_reaction).await?;
        }
        FullEvent::VoiceStateUpdate { old, new } => {
            audit::handle_voice_state_update(ctx, old, new, &data).await?;
            music::handle_voice_state_update(ctx, old, new, &data).await?;
        }
        FullEvent::Message { new_message } => {
            lottery::handle_message(ctx, new_message, &data).await?;
            reactions::handle_message(ctx, new_message, &data).await?;
        }
        _ => {}
    }
    Ok(())
}
