mod audit;
mod lottery;
mod music;
mod reactions;
mod ready;
mod roles;

pub use roles::{ROLE_CHANNEL, ROLE_REACTION};
use serenity::all::{Context, FullEvent};
use serenity::async_trait;
use serenity::prelude::EventHandler;

use crate::prelude::{CommandData, Result};

pub struct DiscordEventHandler;

#[async_trait]
impl EventHandler for DiscordEventHandler {
    async fn dispatch(&self, ctx: &Context, event: &FullEvent) {
        if let Err(err) = dispatch_helper(ctx, event).await {
            eprintln!("Event handler error: {err:?}");
        }
    }
}

async fn dispatch_helper(ctx: &Context, event: &FullEvent) -> Result<()> {
    match event {
        FullEvent::Message { new_message, .. } => {
            let data = ctx.data::<CommandData>();
            lottery::handle_message(ctx, new_message, &data).await?;
            reactions::handle_message(ctx, new_message, &data).await?;
        }
        FullEvent::ReactionAdd { add_reaction, .. } => {
            roles::handle_reaction_add(ctx, add_reaction).await?;
        }
        FullEvent::ReactionRemove {
            removed_reaction, ..
        } => {
            roles::handle_reaction_remove(ctx, removed_reaction).await?;
        }
        FullEvent::Ready { data_about_bot, .. } => {
            ready::handle_ready(ctx, data_about_bot).await?;
        }
        FullEvent::VoiceStateUpdate { old, new, .. } => {
            let data = ctx.data::<CommandData>();
            audit::handle_voice_state_update(ctx, old, new, &data).await?;
            music::handle_voice_state_update(ctx, old, new, &data).await?;
        }
        _ => {}
    }
    Ok(())
}
