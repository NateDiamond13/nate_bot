use chrono::NaiveDateTime;
use sqlx::{query, query_as};

use crate::prelude::{DbExecutor, Error, Result};

/// Entity struct representing a patch notes subscription entry in the database
#[derive(Clone, Debug)]
pub struct PatchNotesSub {
    /// Internal name for the target game
    pub target_game: String,
    /// Guild ID of the subscription
    pub guild_id: String,
    /// Channel ID of the subscription
    pub channel_id: String,
    /// Webhook ID for sending alerts to the subscribed channel
    pub webhook_id: u64,
    /// Webhook token used for authentication
    pub webhook_token: String,
    /// Date and time of creation
    pub created_at: NaiveDateTime,
}

#[derive(Clone, Debug)]
struct PatchNotesSubInternal {
    pub target_game: String,
    pub guild_id: String,
    pub channel_id: String,
    pub webhook_id: String,
    pub webhook_token: String,
    pub created_at: NaiveDateTime,
}

impl TryFrom<PatchNotesSubInternal> for PatchNotesSub {
    fn try_from(value: PatchNotesSubInternal) -> Result<Self> {
        Ok(PatchNotesSub {
            target_game: value.target_game,
            guild_id: value.guild_id,
            channel_id: value.channel_id,
            webhook_id: value.webhook_id.parse::<u64>()?,
            webhook_token: value.webhook_token,
            created_at: value.created_at,
        })
    }

    type Error = Error;
}

/// Struct for creating [`PatchNotesSub`] entries
#[derive(Clone, Debug)]
pub struct CreatePatchNotesSub {
    /// Internal name for the target game
    pub target_game: String,
    /// Guild ID of the subscription
    pub guild_id: String,
    /// Channel ID of the subscription
    pub channel_id: String,
    /// Webhook ID for sending alerts to the subscribed channel
    pub webhook_id: u64,
    /// Webhook token used for authentication
    pub webhook_token: String,
}

/// Get a [`PatchNotesSub`] entry for a given `target_game`, `guild_id`, and `channel_id`
pub async fn get<'a>(
    dbx: impl DbExecutor<'a>,
    target_game: impl Into<String>,
    guild_id: impl Into<String>,
    channel_id: impl Into<String>,
) -> Option<Result<PatchNotesSub>> {
    let result = query_as!(
        PatchNotesSubInternal,
        "SELECT *
        FROM patch_notes_subscriptions
        WHERE target_game = LOWER($1) AND guild_id = $2 AND channel_id = $3;",
        target_game.into(),
        guild_id.into(),
        channel_id.into()
    )
    .fetch_one(dbx)
    .await
    .ok();

    result.map(|r| r.try_into())
}

/// Get all [`PatchNotesSub`] entries from the database for a given `target_game`
pub async fn get_all_for_game<'a>(
    dbx: impl DbExecutor<'a>,
    target_game: impl Into<String>,
) -> Option<Vec<PatchNotesSub>> {
    let result = query_as!(
        PatchNotesSubInternal,
        "SELECT *
        FROM patch_notes_subscriptions
        WHERE target_game = LOWER($1)
        ORDER BY guild_id, channel_id;",
        target_game.into(),
    )
    .fetch_all(dbx)
    .await
    .ok();

    result.map(|v| v.iter().flat_map(|r| r.clone().try_into()).collect())
}

/// Insert a new [`CreatePatchNotesSub`] into the database
pub async fn insert<'a>(
    dbx: impl DbExecutor<'a>,
    create_patch_notes_sub: &CreatePatchNotesSub,
) -> Result<bool> {
    let insert_result = query!(
        "INSERT INTO patch_notes_subscriptions (target_game, guild_id, channel_id, webhook_id,
            webhook_token)
        VALUES (LOWER($1), $2, $3, $4, $5)
        ON CONFLICT (target_game, guild_id, channel_id)
            DO UPDATE SET
                webhook_id = $4,
                webhook_token = $5;",
        create_patch_notes_sub.target_game,
        create_patch_notes_sub.guild_id,
        create_patch_notes_sub.channel_id,
        create_patch_notes_sub.webhook_id.to_string(),
        create_patch_notes_sub.webhook_token,
    )
    .execute(dbx)
    .await?;

    Ok(insert_result.rows_affected() > 0)
}

/// Remove a [`PatchNotesSub`] entry for a given `target_game`, `guild_id`, and `channel_id`
pub async fn remove<'a>(
    dbx: impl DbExecutor<'a>,
    target_game: impl Into<String>,
    guild_id: impl Into<String>,
    channel_id: impl Into<String>,
) -> Result<bool> {
    let remove_result = query!(
        "DELETE FROM patch_notes_subscriptions
        WHERE target_game = LOWER($1) AND guild_id = $2 AND channel_id = $3;",
        target_game.into(),
        guild_id.into(),
        channel_id.into()
    )
    .execute(dbx)
    .await?;

    Ok(remove_result.rows_affected() > 0)
}
