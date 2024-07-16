mod commands;
mod db;
mod error;
mod events;
mod prelude;
mod utils;

use prelude::{CommandData, Error, HttpClient, Result};

use poise::{
    builtins, ApplicationContext, Context, Framework, FrameworkOptions, PrefixContext,
    PrefixFrameworkOptions,
};
use serenity::all::ActivityData;
use serenity::prelude::{Client, GatewayIntents};
use songbird::SerenityInit;

#[tokio::main]
async fn main() -> Result<()> {
    // Load bot token from the environment
    let env_vars = utils::load_env()?;
    let env = env_vars.clone();

    // Set up database connection pool
    let pool = db::get_connection_pool(&env_vars.database_url).await?;

    // Set up http client for songbird
    let http_client = HttpClient::new();

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
            prefix: Some(env_vars.command_prefix),
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
            Box::pin(async move { println!("Error occurred during command: {:?}", err) })
        },
        // Ignore commands from bots
        command_check: Some(|ctx| Box::pin(async move { Ok(!ctx.author().bot) })),
        // Handle events
        event_handler: |ctx, event, framework, data| {
            Box::pin(events::event_handler(ctx, event, framework, data))
        },
        ..Default::default()
    };
    let framework: Framework<CommandData, Error> = Framework::builder()
        // Register built-in commands to Discord Integrations page
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(CommandData {
                    env,
                    pool,
                    http_client,
                })
            })
        })
        .options(options)
        .build();

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
    let mut client = Client::builder(env_vars.discord_token, intents)
        .framework(framework)
        .register_songbird()
        .activity(ActivityData::custom(env_vars.custom_status))
        .await?;

    // Start listening for events with an automatically determined number of shards
    if let Err(why) = client.start_autosharded().await {
        println!("Client error: {why:?}");
    }
    Ok(())
}
