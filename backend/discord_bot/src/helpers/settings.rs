use std::fmt;

use database::bot_settings::{self, BotSettings};
use poise::ChoiceParameter;
use serenity::all::{ActivityData, CacheHttp, Context, OnlineStatus};

use crate::prelude::{CommandData, Error, Result};

#[derive(ChoiceParameter, Clone, Debug)]
pub enum BotStatus {
    #[name = "dnd"]
    DoNotDisturb,
    #[name = "idle"]
    Idle,
    #[name = "invisible"]
    Invisible,
    #[name = "offline"]
    Offline,
    #[name = "online"]
    Online,
}

impl BotStatus {
    pub fn from_str(input: &str) -> Self {
        match input {
            "dnd" => BotStatus::DoNotDisturb,
            "idle" => BotStatus::Idle,
            "invisible" => BotStatus::Invisible,
            "offline" => BotStatus::Offline,
            "online" => BotStatus::Online,
            _ => BotStatus::Online,
        }
    }
}

impl fmt::Display for BotStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl From<BotStatus> for OnlineStatus {
    fn from(val: BotStatus) -> Self {
        match val {
            BotStatus::DoNotDisturb => OnlineStatus::DoNotDisturb,
            BotStatus::Idle => OnlineStatus::Idle,
            BotStatus::Invisible => OnlineStatus::Invisible,
            BotStatus::Offline => OnlineStatus::Offline,
            BotStatus::Online => OnlineStatus::Online,
        }
    }
}

/// Initialize bot settings from stored settings in database (or default if they don't exist yet)
pub async fn initialize_bot(context: &Context, data: &CommandData) -> Result<()> {
    let Some(app_id) = context.http().application_id() else {
        return Err(Error::InvalidApplicationID);
    };
    let application_id = app_id.to_string();

    let mut conn = data.pool.get_connection().await?;

    // If the settings already exist in the database, use them to set the bot presence
    if let Some(cur_settings) = bot_settings::get(conn.as_mut(), &application_id).await {
        set_presence(context, cur_settings);
        log::info!("Initialized bot settings from database");
    } else {
        // Otherwise, add a new entry in the database for this bot
        let name = context.http().get_current_user().await?.name.to_string();
        let new_settings = BotSettings {
            application_id,
            name,
            status: BotStatus::Online.to_string(),
            activity: None,
        };

        if let Some(updated_settings) = bot_settings::upsert(conn.as_mut(), new_settings).await {
            set_presence(context, updated_settings);
            log::info!("Added new bot settings to database");
        } else {
            log::warn!("Unable to add new bot settings to database");
        }
    }

    Ok(())
}

/// Update the bot's current activity
pub async fn update_activity(
    context: &Context,
    data: &CommandData,
    activity: Option<String>,
) -> Result<bool> {
    let Some(app_id) = context.http().application_id() else {
        return Err(Error::InvalidApplicationID);
    };
    let application_id = app_id.to_string();

    let mut conn = data.pool.get_connection().await?;
    if let Some(updated_settings) =
        bot_settings::update_activity(conn.as_mut(), application_id, activity).await
    {
        set_presence(context, updated_settings);
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Update the bot's current status
pub async fn update_status(
    context: &Context,
    data: &CommandData,
    status: &BotStatus,
) -> Result<bool> {
    let Some(app_id) = context.http().application_id() else {
        return Err(Error::InvalidApplicationID);
    };
    let application_id = app_id.to_string();

    let mut conn = data.pool.get_connection().await?;
    if let Some(updated_settings) =
        bot_settings::update_status(conn.as_mut(), application_id, status.to_string()).await
    {
        set_presence(context, updated_settings);
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Set the bot's presence (activity & status) according to the given [`BotSettings`]
fn set_presence(context: &Context, settings: BotSettings) {
    let activity = settings.activity.map(ActivityData::custom);
    let status = BotStatus::from_str(&settings.status);
    context.set_presence(activity, status.into());
}
