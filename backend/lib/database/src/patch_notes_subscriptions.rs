use chrono::NaiveDateTime;
use sqlx::{PgPool, query, query_as};

use crate::prelude::{Error, Result};

#[derive(Debug)]
pub struct PatchNotesSub {
    pub target_game: String,
    pub guild_id: String,
    pub channel_id: String,
    pub webhook_id: u64,
    pub webhook_token: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub struct CreatePatchNotesSub {
    pub target_game: String,
    pub guild_id: String,
    pub channel_id: String,
    pub webhook_id: u64,
    pub webhook_token: String,
}

pub async fn get(
    conn: &PgPool,
    target_game: impl Into<String>,
    guild_id: impl Into<String>,
    channel_id: impl Into<String>,
) -> Option<Result<PatchNotesSub>> {
    let result = query_as!(
        PatchNotesSubInternal,
        "SELECT * FROM patch_notes_subscriptions
        WHERE target_game = LOWER($1) AND guild_id = $2 AND channel_id = $3;",
        target_game.into(),
        guild_id.into(),
        channel_id.into()
    )
    .fetch_one(conn)
    .await
    .ok();
    result.map(|r| r.try_into())
}

pub async fn get_all_for_game(
    conn: &PgPool,
    target_game: impl Into<String>,
) -> Option<Vec<PatchNotesSub>> {
    let result = query_as!(
        PatchNotesSubInternal,
        "SELECT * FROM patch_notes_subscriptions
        WHERE target_game = LOWER($1)
        ORDER BY guild_id, channel_id;",
        target_game.into(),
    )
    .fetch_all(conn)
    .await
    .ok();
    result.map(|v| v.iter().flat_map(|r| r.clone().try_into()).collect())
}

pub async fn insert(conn: &PgPool, create_patch_notes_sub: &CreatePatchNotesSub) -> Result<()> {
    query!(
        "INSERT INTO patch_notes_subscriptions
            (target_game, guild_id, channel_id, webhook_id, webhook_token)
        VALUES (LOWER($1), $2, $3, $4, $5)
        ON CONFLICT (target_game, guild_id, channel_id) DO UPDATE
        SET webhook_id = $4, webhook_token = $5;",
        create_patch_notes_sub.target_game,
        create_patch_notes_sub.guild_id,
        create_patch_notes_sub.channel_id,
        create_patch_notes_sub.webhook_id.to_string(),
        create_patch_notes_sub.webhook_token,
    )
    .execute(conn)
    .await?;
    Ok(())
}

pub async fn remove(
    conn: &PgPool,
    target_game: impl Into<String>,
    guild_id: impl Into<String>,
    channel_id: impl Into<String>,
) -> Result<()> {
    query!(
        "DELETE FROM patch_notes_subscriptions
        WHERE target_game = LOWER($1) AND guild_id = $2 AND channel_id = $3;",
        target_game.into(),
        guild_id.into(),
        channel_id.into()
    )
    .execute(conn)
    .await?;
    Ok(())
}
