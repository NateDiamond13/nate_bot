use chrono::NaiveDateTime;
use sqlx::{query, query_as};

use crate::prelude::{DbExecutor, Result};

/// Entity struct representing a picture entry in the database
#[derive(Clone, Debug)]
pub struct Picture {
    /// Name of the picture
    pub name: String,
    /// ID of the guild that added the picture
    pub guild_id: String,
    /// URL to the picture
    pub url: String,
    /// ID of the user that added the picture
    pub added_by_user: String,
    /// Whether the picture is explicit
    pub is_nsfw: bool,
    /// Date and time of creation
    pub created_at: NaiveDateTime,
}

/// Struct for creating [`Picture`] entries
#[derive(Clone, Debug)]
pub struct CreatePicture {
    /// Name of the picture
    pub name: String,
    /// ID of the guild that added the picture
    pub guild_id: String,
    /// URL to the picture
    pub url: String,
    /// ID of the user that added the picture
    pub added_by_user: String,
    /// Whether the picture is explicit
    pub is_nsfw: bool,
}

/// Get a [`Picture`] entry from the database with the given `name` and `guild_id`
pub async fn get<'a>(
    dbx: impl DbExecutor<'a>,
    name: impl Into<String>,
    guild_id: impl Into<String>,
) -> Option<Picture> {
    query_as!(
        Picture,
        "SELECT *
        FROM pictures
        WHERE name = $1 AND guild_id = $2;",
        name.into(),
        guild_id.into()
    )
    .fetch_one(dbx)
    .await
    .ok()
}

/// Get all [`Picture`] entries from the database for a given `guild_id`
pub async fn get_all<'a>(
    dbx: impl DbExecutor<'a>,
    guild_id: impl Into<String>,
) -> Option<Vec<Picture>> {
    query_as!(
        Picture,
        "SELECT *
        FROM pictures
        WHERE guild_id = $1
        ORDER BY name;",
        guild_id.into()
    )
    .fetch_all(dbx)
    .await
    .ok()
}

/// Get a random [`Picture`] entry from the database for a given `guild_id` and `is_nsfw` flag
pub async fn get_random<'a>(
    dbx: impl DbExecutor<'a>,
    guild_id: impl Into<String>,
    is_nsfw: Option<bool>,
) -> Option<Picture> {
    if let Some(flag_nsfw) = is_nsfw {
        query_as!(
            Picture,
            "SELECT *
            FROM pictures
            WHERE guild_id = $1 AND is_nsfw = $2
            ORDER BY random()
            LIMIT 1;",
            guild_id.into(),
            flag_nsfw
        )
        .fetch_one(dbx)
        .await
        .ok()
    } else {
        query_as!(
            Picture,
            "SELECT *
            FROM pictures
            WHERE guild_id = $1
            ORDER BY random()
            LIMIT 1;",
            guild_id.into()
        )
        .fetch_one(dbx)
        .await
        .ok()
    }
}

/// Insert a new [`CreatePicture`] into the database
pub async fn insert<'a>(dbx: impl DbExecutor<'a>, create_pic: &CreatePicture) -> Result<bool> {
    let insert_result = query!(
        "INSERT INTO pictures (name, guild_id, url, added_by_user, is_nsfw)
        VALUES ($1, $2, $3, $4, $5);",
        create_pic.name,
        create_pic.guild_id,
        create_pic.url,
        create_pic.added_by_user,
        create_pic.is_nsfw
    )
    .execute(dbx)
    .await?;

    Ok(insert_result.rows_affected() > 0)
}

/// Remove a [`Picture`] from the database with the given `name` and `guild_id`
pub async fn remove<'a>(
    dbx: impl DbExecutor<'a>,
    name: impl Into<String>,
    guild_id: impl Into<String>,
) -> Result<bool> {
    let remove_result = query!(
        "DELETE FROM pictures
        WHERE name = $1 AND guild_id = $2;",
        name.into(),
        guild_id.into()
    )
    .execute(dbx)
    .await?;

    Ok(remove_result.rows_affected() > 0)
}
