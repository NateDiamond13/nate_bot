use crate::prelude::Result;

use chrono::NaiveDateTime;
use sqlx::{FromRow, PgPool, query, query_as};

#[derive(Debug, FromRow)]
pub struct PatchNotes {
    pub target_game: String,
    pub link: String,
    pub title: String,
    pub content: String,
    pub game_title: String,
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct CreatePatchNotes {
    pub target_game: String,
    pub patch_id: String,
    pub link: String,
    pub title: String,
    pub content: String,
    pub posted_at: NaiveDateTime,
}

pub async fn get_latest(conn: &PgPool, target_game: impl Into<String>) -> Option<PatchNotes> {
    query_as!(
        PatchNotes,
        "WITH latest_patch AS (
            SELECT * FROM patch_notes
            WHERE target_game = LOWER($1)
            ORDER BY posted_at DESC
            LIMIT 1
        )
        SELECT target_game, link, title, content, game_title, thumbnail_url
        FROM latest_patch lp JOIN patch_game_info pgi
            ON lp.target_game = pgi.internal_name;",
        target_game.into()
    )
    .fetch_one(conn)
    .await
    .ok()
}

pub async fn insert(conn: &PgPool, create_patch_notes: &CreatePatchNotes) -> Result<u64> {
    Ok(query!(
        "INSERT INTO patch_notes (target_game, patch_id, link, title, content, posted_at)
        VALUES (LOWER($1), $2, $3, $4, $5, $6)
        ON CONFLICT (target_game, patch_id) DO NOTHING;",
        create_patch_notes.target_game,
        create_patch_notes.patch_id,
        create_patch_notes.link,
        create_patch_notes.title,
        create_patch_notes.content,
        create_patch_notes.posted_at
    )
    .execute(conn)
    .await?
    .rows_affected())
}
