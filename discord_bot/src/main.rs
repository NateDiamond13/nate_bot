use crate::prelude::*;

mod commands;
mod error;
mod prelude;
mod utils;

use dotenv::dotenv;
use std::env;

use commands::ping::PingHandler;
use serenity::prelude::*;

struct EnvVariables {
    discord_token: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Login with a bot token from the environment
    let env_vars = load_env()?;
    let token = env_vars.discord_token;

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    println!("Starting bot...");
    let mut client = Client::builder(&token, intents)
        .event_handler(PingHandler)
        .await?;
    println!("Bot started!");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }

    Ok(())
}

fn load_env() -> Result<EnvVariables> {
    dotenv()?;
    let discord_token = env::var("DISCORD_TOKEN")?;

    Ok(EnvVariables { discord_token })
}
