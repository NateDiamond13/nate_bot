use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::{Context, EventHandler};

pub struct EventWatcher {
    pub shard_count: u32,
}

#[async_trait]
impl EventHandler for EventWatcher {
    async fn ready(&self, ctx: Context, ready: Ready) {
        if self.shard_count == 1 {
            println!("{} is connected!", ready.user.name);
        } else {
            println!(
                "{} (Shard: {} of {}) is connected!",
                ready.user.name, ctx.shard_id, self.shard_count
            );
        }
    }
}
