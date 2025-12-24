use chrono::NaiveDateTime;
use sqlx::{query, query_as};

use crate::prelude::{DbExecutor, Result};

/// Entity struct representing a patch notes entry in the database
#[derive(Clone, Debug)]
pub struct PatchNotes {
    /// Internal name for the target game
    pub target_game: String,
    /// Link to the patch notes page
    pub link: String,
    /// Title of the patch notes
    pub title: String,
    /// Content of the patch notes
    pub content: String,
    /// Official title of the game
    pub game_title: String,
    /// Steam App ID for the game
    pub steam_app_id: Option<String>,
    /// Thumbnail URL for the game
    pub thumbnail_url: Option<String>,
}

/// Struct for creating [`PatchNotes`] entries
#[derive(Clone, Debug)]
pub struct CreatePatchNotes {
    /// Internal name for the target game
    pub target_game: String,
    /// ID of the patch notes
    pub patch_id: String,
    /// Link to the patch notes page
    pub link: String,
    /// Title of the patch notes
    pub title: String,
    /// Content of the patch notes
    pub content: String,
    /// Date and time it was posted
    pub posted_at: NaiveDateTime,
}

/// Metadata for a Steam game's latest patch notes entry
#[derive(Clone, Debug)]
pub struct SteamPatchMetadata {
    /// Internal name for the target game
    pub target_game: String,
    /// Steam App ID for the game
    pub steam_app_id: String,
    /// ID of the latest patch notes (if it exists)
    pub latest_patch_id: Option<String>,
    /// Date and time the latest patch notes were posted (if it exists)
    pub latest_posted_at: Option<NaiveDateTime>,
}

/// Get the latest [`PatchNotes`] entry from the database for a given `target_game`
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

/// Insert a new [`CreatePatchNotes`] into the database
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

/// Get the latest [`SteamPatchMetadata`] from the database for all games with a Steam App ID
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
