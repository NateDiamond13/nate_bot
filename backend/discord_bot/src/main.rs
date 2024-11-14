mod commands;
mod events;
mod helpers;
mod prelude;

use std::sync::Arc;

use env_logger::Env;
use poise::serenity_prelude::ClientBuilder;
use poise::{
    ApplicationContext, Context, Framework, FrameworkOptions, PrefixContext, PrefixFrameworkOptions,
};
use prelude::{CommandData, HttpClient, Result};
use serenity::all::ActivityData;
use serenity::prelude::GatewayIntents;
use songbird::Songbird;

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

    let data = Arc::new(CommandData {
        env,
        pool,
        http_client,
        songbird_manager,
    });

    // Set up poise framework with options
    let options = FrameworkOptions {
        commands: vec![
            commands::help(),
            commands::ping(),
            commands::pictures(),
            commands::purge(),
            commands::roles(),
            commands::roll(),
            commands::music::play(),
            commands::music::skip(),
            commands::music::stop(),
        ],
        // Allows prefix commands to be executed
        prefix_options: PrefixFrameworkOptions {
            prefix: Some(env_vars.command_prefix.into()),
            ..Default::default()
        },
        // Logs which commands are executed and by whom
        pre_command: |ctx| {
            Box::pin(async move {
                match ctx {
                    Context::Prefix(PrefixContext { msg, .. }) => {
                        println!(
                            "User \"{}\" executed prefix command: [\"{}\"]",
                            msg.author.name, msg.content
                        );
                    }
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
                }
            })
        },
        // Logs on command errors
        on_error: |err| {
            Box::pin(async move { println!("Error occurred during command: {:#?}", err) })
        },
        // Ignore commands from bots
        command_check: Some(|ctx| Box::pin(async move { Ok(!ctx.author().bot()) })),
        // Handle events
        event_handler: |framework_context, event| {
            Box::pin(events::event_handler(framework_context, event))
        },
        ..Default::default()
    };

    let framework = Framework::new(options);

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MODERATION
        | GatewayIntents::GUILD_VOICE_STATES
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot
    println!("Starting bot...");
    let mut client = ClientBuilder::new(&env_vars.discord_token, intents)
        .voice_manager::<Songbird>(data.songbird_manager.clone())
        .framework(framework)
        .activity(ActivityData::custom(env_vars.custom_status))
        .data(data)
        .await?;

    // Start listening for events with an automatically determined number of shards
    if let Err(why) = client.start_autosharded().await {
        println!("Client error: {why:?}");
    }
    Ok(())
}
