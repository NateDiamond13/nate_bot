use serenity::all::{CacheHttp, Context, Ready};

use crate::commands;
use crate::prelude::Result;

pub async fn handle_ready(ctx: &Context, event: &Ready) -> Result<()> {
    let shard_id = ctx.shard_id.get();
    println!("{} (Shard {}) is connected!", event.user.name, &shard_id);

    if shard_id == 0 {
        let commands = commands::get_commands();
        poise::builtins::register_globally(ctx.http(), &commands).await?;
    }

    Ok(())
}
