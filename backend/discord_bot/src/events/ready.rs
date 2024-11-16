use crate::prelude::{CommandData, Error, Result};

use poise::FrameworkContext;
use serenity::all::{CacheHttp, Ready};

pub async fn handle_ready(
    framework_context: &FrameworkContext<'_, CommandData, Error>,
    event: &Ready,
) -> Result<()> {
    let ctx = framework_context.serenity_context;
    let shard_id = ctx.shard_id.get();
    println!("{} (Shard {}) is connected!", event.user.name, &shard_id);

    if shard_id == 0 {
        let commands = &framework_context.options.commands;
        poise::builtins::register_globally(ctx.http(), commands).await?;
    }

    Ok(())
}
