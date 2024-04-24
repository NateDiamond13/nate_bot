use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::prelude::{Context, EventHandler};

use crate::utils;
use discord_bot::COMMAND_PREFIX;

pub struct PingHandler;

#[async_trait]
impl EventHandler for PingHandler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }
        dbg!(&ctx);
        dbg!(&msg);

        if msg.content == format!("{COMMAND_PREFIX}ping") {
            utils::send_message(&ctx, &msg, "Pong!").await;
        }
    }
}
