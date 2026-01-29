use sqlx::query_as;

use crate::prelude::DbExecutor;

/// Entity struct representing bot settings in the database
#[derive(Clone, Debug)]
pub struct BotSettings {
    /// Application ID of the bot
    pub application_id: String,
    /// Bot name
    pub name: String,
    /// Current status (online, idle, etc.)
    pub status: String,
    /// Current activity (optional)
    pub activity: Option<String>,
}

/// Get the current [`BotSettings`] entry from the database for a given `application_id`
pub async fn get<'a>(
    dbx: impl DbExecutor<'a>,
    application_id: impl Into<String>,
) -> Option<BotSettings> {
    query_as!(
        BotSettings,
        "SELECT application_id, name, status, activity
        FROM bot_settings
        WHERE application_id = $1;",
        application_id.into()
    )
    .fetch_one(dbx)
    .await
    .ok()
}

/// Insert/update the current [`BotSettings`] for a bot, and return the updated settings
pub async fn upsert<'a>(
    dbx: impl DbExecutor<'a>,
    bot_settings: BotSettings,
) -> Option<BotSettings> {
    query_as!(
        BotSettings,
        "INSERT INTO bot_settings (application_id, name, status, activity)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT (application_id)
            DO UPDATE SET
                status = $3,
                activity = $4,
                updated_at = NOW()
        RETURNING application_id, name, status, activity;",
        bot_settings.application_id,
        bot_settings.name,
        bot_settings.status,
        bot_settings.activity,
    )
    .fetch_one(dbx)
    .await
    .ok()
}

/// Update a bot's activity
pub async fn update_activity<'a>(
    dbx: impl DbExecutor<'a>,
    application_id: impl Into<String>,
    activity: Option<String>,
) -> Option<BotSettings> {
    query_as!(
        BotSettings,
        "UPDATE bot_settings
        SET activity = $2,
            updated_at = NOW()
        WHERE application_id = $1
        RETURNING application_id, name, status, activity;",
        application_id.into(),
        activity,
    )
    .fetch_one(dbx)
    .await
    .ok()
}

/// Update a bot's status
pub async fn update_status<'a>(
    dbx: impl DbExecutor<'a>,
    application_id: impl Into<String>,
    status: impl Into<String>,
) -> Option<BotSettings> {
    query_as!(
        BotSettings,
        "UPDATE bot_settings
        SET status = $2,
            updated_at = NOW()
        WHERE application_id = $1
        RETURNING application_id, name, status, activity;",
        application_id.into(),
        status.into(),
    )
    .fetch_one(dbx)
    .await
    .ok()
}
