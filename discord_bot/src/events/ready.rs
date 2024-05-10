use crate::prelude::{CommandData, Result};

use serenity::all::Ready;
use serenity::prelude::Context;

pub fn handle_ready(ctx: &Context, event: &Ready, data: &CommandData) -> Result<()> {
    if data.env.shard_count == 1 {
        println!("{} is connected!", event.user.name);
    } else {
        println!(
            "{} (Shard: {} of {}) is connected!",
            event.user.name, ctx.shard_id, data.env.shard_count
        );
    }
    Ok(())
}
