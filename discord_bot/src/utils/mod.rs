use serenity::model::channel::Message;
use serenity::prelude::Context;

pub async fn send_message(ctx: &Context, msg: &Message, content: &str) {
    if let Err(why) = msg.channel_id.say(&ctx.http, content).await {
        println!("Error sending message: {why:?}");
    }
}
