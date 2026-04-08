//! Discord Bot
//!
//! Application that creates a bot that will connect to Discord guilds and interact with users.

mod commands;
mod events;
mod helpers;
mod prelude;

use std::str::FromStr;
use std::sync::Arc;

use database::DbPool;
use events::DiscordEventHandler;
use poise::serenity_prelude::ClientBuilder;
use poise::{ApplicationContext, Context, Framework, FrameworkOptions};
use prelude::{CommandData, Result};
use serenity::all::Token;
use serenity::prelude::GatewayIntents;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    // Register logger
    utils::init_logger();

    // Run the bot
    run_bot().await
}

async fn run_bot() -> Result<()> {
    // Load bot token from the environment
    let env_vars = utils::get_config_safe()?;
    let env = env_vars.clone();

    // Register crypto provider
    helpers::crypto::install_default();

    // Set up database connection pool
    let pool = DbPool::new(&env_vars.database_url).await?;

    // Set up the data accessible for every command
    let data = Arc::new(CommandData { env, pool });

    // Set up poise framework with options
    let options = FrameworkOptions {
        // Defines all available commands
        commands: commands::get_commands(),
        // Logs which commands are executed and by whom
        pre_command: |ctx| {
            Box::pin(async move {
                if let Context::Application(ApplicationContext {
                    interaction,
                    command,
                    ..
                }) = ctx
                {
                    log::info!(
                        "User \"{}\" executed slash command: [\"/{}\"]",
                        interaction.user.name,
                        command.qualified_name
                    );
                }
            })
        },
        // Logs on command errors
        on_error: |err| {
            Box::pin(async move { log::info!("Error occurred during command: {err:#?}") })
        },
        // Ignore commands from bots
        command_check: Some(|ctx| Box::pin(async move { Ok(!ctx.author().bot()) })),
        ..Default::default()
    };

    // Build the poise framework from the given options
    let framework = Framework::builder().options(options).build();

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MODERATION
        | GatewayIntents::GUILD_WEBHOOKS
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILD_PRESENCES
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot
    log::info!("Starting bot...");
    let discord_token = Token::from_str(&env_vars.discord_token)?;
    let mut client = ClientBuilder::new(discord_token, intents)
        .framework(Box::new(framework))
        .data::<CommandData>(data)
        .event_handler(Arc::new(DiscordEventHandler))
        .await?;

    // Start listening for events with an automatically determined number of shards
    tokio::select! {
        res = client.start_autosharded() => {
            if let Err(why) = res {
                log::error!("Client Error: {why:?}");
            }
        }
        _ = signal::ctrl_c() => {
            log::info!("Ctrl-C received, shutting down");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use crate::prelude::Result;
    use crate::run_bot;

    #[ignore]
    #[test(tokio::test)]
    async fn test_discord_bot() -> Result<()> {
        run_bot().await?;

        Ok(())
    }
}
