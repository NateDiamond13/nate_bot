use serenity::all::{CacheHttp, Context, Ready};

use crate::prelude::{CommandData, Result};
use crate::{commands, lavalink};

pub async fn handle_ready(ctx: &Context, event: &Ready) -> Result<()> {
    let shard_id = ctx.shard_id.get();
    log::info!("{} (Shard {}) is connected!", event.user.name, &shard_id);

    if shard_id == 0 {
        let commands = commands::get_commands();
        poise::builtins::register_globally(ctx.http(), &commands).await?;

        let user_id = ctx.cache.current_user().id;
        let session_id = event.session_id.to_string();

        // Update Lavalink client in context data
        {
            let data = ctx.data::<CommandData>();

            let env_vars = &data.env;
            let lavalink_client = lavalink::get_client(
                &env_vars.lavalink_hostname,
                &env_vars.lavalink_password,
                user_id.get(),
                session_id,
            )
            .await;

            log::info!("Lavalink client created");

            let mut client_lock = data.lavalink_client.try_lock()?;
            *client_lock = lavalink_client;

            log::info!("Updated Lavalink client with bot user ID");
        }
    }

    Ok(())
}
