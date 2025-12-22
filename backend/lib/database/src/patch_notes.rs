use chrono::NaiveDateTime;
use sqlx::{FromRow, query, query_as};

use crate::prelude::{DbExecutor, Result};

#[derive(Debug, FromRow)]
pub struct PatchNotes {
    pub target_game: String,
    pub link: String,
    pub title: String,
    pub content: String,
    pub game_title: String,
    pub steam_app_id: Option<String>,
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

#[derive(Debug, FromRow)]
pub struct SteamPatchMetadata {
    pub target_game: String,
    pub steam_app_id: String,
    pub latest_patch_id: Option<String>,
    pub latest_posted_at: Option<NaiveDateTime>,
}

pub async fn get_latest<'a>(
    dbx: impl DbExecutor<'a>,
    target_game: impl Into<String>,
) -> Option<PatchNotes> {
    query_as!(
        PatchNotes,
        "WITH latest_patch AS (
            SELECT *
            FROM patch_notes
            WHERE target_game = LOWER($1)
            ORDER BY posted_at DESC
            LIMIT 1
        )
        SELECT target_game, link, title, content, game_title, steam_app_id, thumbnail_url
        FROM latest_patch lp
            JOIN patch_game_info pgi
                ON lp.target_game = pgi.internal_name;",
        target_game.into()
    )
    .fetch_one(dbx)
    .await
    .ok()
}

pub async fn insert<'a>(
    dbx: impl DbExecutor<'a>,
    create_patch_notes: &CreatePatchNotes,
) -> Result<bool> {
    let insert_result = query!(
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
    .execute(dbx)
    .await?;

    Ok(insert_result.rows_affected() > 0)
}

pub async fn get_latest_steam_patches<'a>(
    dbx: impl DbExecutor<'a>,
) -> Result<Vec<SteamPatchMetadata>> {
    let metadata_result = query_as!(
        SteamPatchMetadata,
        r#"WITH steam_apps AS (
            SELECT internal_name AS target_game, steam_app_id
            FROM patch_game_info
            WHERE steam_app_id IS NOT NULL
        ),
        latest_patches AS (
            SELECT DISTINCT ON (target_game) target_game, patch_id, posted_at
            FROM patch_notes
            ORDER BY target_game, posted_at DESC
        )
        SELECT target_game, steam_app_id AS "steam_app_id!", patch_id AS latest_patch_id,
            posted_at AS latest_posted_at
        FROM steam_apps
            LEFT JOIN latest_patches USING (target_game)
        ORDER BY target_game;"#
    )
    .fetch_all(dbx)
    .await?;

    Ok(metadata_result)
}
