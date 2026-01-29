use serenity::all::{CacheHttp, Context, Ready};

use crate::commands;
use crate::helpers::settings;
use crate::prelude::{CommandData, Result};

pub async fn handle_ready(ctx: &Context, event: &Ready, data: &CommandData) -> Result<()> {
    let shard_id = ctx.shard_id.get();
    log::info!("{} (Shard {}) is connected!", event.user.name, &shard_id);

    if shard_id == 0 {
        // Globally register commands
        let commands = commands::get_commands();
        poise::builtins::register_globally(ctx.http(), &commands).await?;

        // Initialize saved settings
        settings::initialize_bot(ctx, data).await?;
    }

    Ok(())
}
