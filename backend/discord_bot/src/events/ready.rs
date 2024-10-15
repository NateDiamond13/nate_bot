use crate::prelude::Result;

use serenity::all::Ready;
use serenity::prelude::Context;

pub fn handle_ready(ctx: &Context, event: &Ready) -> Result<()> {
    println!("{} (Shard {}) is connected!", event.user.name, ctx.shard_id);
    Ok(())
}
