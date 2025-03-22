mod commands;
mod events;
mod helpers;
mod prelude;
mod services;

use std::str::FromStr;
use std::sync::Arc;

use env_logger::Env;
use events::DiscordEventHandler;
use poise::serenity_prelude::ClientBuilder;
use poise::{ApplicationContext, Context, Framework, FrameworkOptions};
use prelude::{CommandData, HttpClient, Result};
use serenity::all::{ActivityData, Token};
use serenity::prelude::GatewayIntents;
use songbird::Songbird;
use tokio::signal;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("warn")).init();

    // Load bot token from the environment
    let env_vars = utils::get_env_variables();
    let env = env_vars.clone();

    // Set up database connection pool
    let pool = database::get_connection_pool(&env_vars.database_url).await?;

    // Set up http client and manager for songbird
    let http_client = HttpClient::new();
    let songbird_manager = songbird::Songbird::serenity();

    // Set up the data accessible for every command
    let data = Arc::new(CommandData {
        env,
        pool,
        http_client,
        songbird_manager,
    });

    // Set up poise framework with options
    let options = FrameworkOptions {
        // Defines all available commands
        commands: commands::get_commands(),
        // Logs which commands are executed and by whom
        pre_command: |ctx| {
            Box::pin(async move {
                match ctx {
                    Context::Application(ApplicationContext {
                        interaction,
                        command,
                        ..
                    }) => {
                        println!(
                            "User \"{}\" executed slash command: [\"/{}\"]",
                            interaction.user.name, command.qualified_name
                        );
                    }
                    _ => {}
                }
            })
        },
        // Logs on command errors
        on_error: |err| {
            Box::pin(async move { println!("Error occurred during command: {:#?}", err) })
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
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot
    println!("Starting bot...");
    let discord_token = Token::from_str(&env_vars.discord_token)?;
    let mut client = ClientBuilder::new(discord_token, intents)
        .voice_manager::<Songbird>(data.songbird_manager.clone())
        .framework(framework)
        .activity(ActivityData::custom(env_vars.custom_status))
        .data(data)
        .event_handler(DiscordEventHandler)
        .await?;

    // Start listening for events with an automatically determined number of shards
    tokio::select! {
        res = client.start_autosharded() => {
            if let Err(why) = res {
                eprintln!("Client Error: {why:?}");
            }
        }
        _ = signal::ctrl_c() => {
            eprintln!("Ctrl-C received, shutting down");
        }
    }
    Ok(())
}
