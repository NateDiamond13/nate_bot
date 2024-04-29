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
            ready::handle_ready(ctx, data_about_bot, data)?;
        }
        FullEvent::ReactionAdd { add_reaction } => {
            roles::handle_reaction_add(ctx, add_reaction).await?;
        }
        FullEvent::ReactionRemove { removed_reaction } => {
            roles::handle_reaction_remove(ctx, removed_reaction).await?;
        }
        _ => {}
    }
    Ok(())
}
