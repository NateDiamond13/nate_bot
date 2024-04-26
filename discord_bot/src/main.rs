mod commands;
mod error;
mod events;
mod prelude;
mod utils;

use events::EventWatcher;
use prelude::{CommandData, Error, Result};
use utils::EnvVariables;

use poise::{builtins, Framework, FrameworkOptions, PrefixFrameworkOptions};
use serenity::prelude::{Client, GatewayIntents};

#[tokio::main]
async fn main() -> Result<()> {
    // Load bot token from the environment
    let EnvVariables {
        command_prefix,
        discord_token,
        shard_count,
    } = utils::load_env()?;

    // Set up poise framework with options
    let options = FrameworkOptions {
        commands: vec![commands::help(), commands::ping(), commands::roll()],
        prefix_options: PrefixFrameworkOptions {
            prefix: Some(command_prefix),
            ..Default::default()
        },
        // Ignore commands from bots
        command_check: Some(|ctx| Box::pin(async move { Ok(!ctx.author().bot) })),
        ..Default::default()
    };
    let framework: Framework<CommandData, Error> = Framework::builder()
        // Register built-in commands to Discord Integrations page
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(CommandData)
            })
        })
        .options(options)
        .build();

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot
    println!("Starting bot...");
    let mut client = Client::builder(discord_token, intents)
        .framework(framework)
        .event_handler(EventWatcher { shard_count })
        .await?;

    // Start listening for events by starting a limited number of shards
    if let Err(why) = client.start_shards(shard_count).await {
        println!("Client error: {why:?}");
    }

    Ok(())
}
