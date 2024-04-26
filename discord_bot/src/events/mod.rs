use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::{Context, EventHandler};

pub struct EventWatcher;

#[async_trait]
impl EventHandler for EventWatcher {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!(
            "{} (Shard: {}) is connected!",
            ready.user.name, ctx.shard_id
        );
    }
}
