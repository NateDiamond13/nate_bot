use chrono::NaiveDateTime;
use sqlx::{FromRow, query, query_as};

use crate::prelude::{DbExecutor, Result};

#[derive(Debug, FromRow)]
pub struct Picture {
    pub name: String,
    pub guild_id: String,
    pub url: String,
    pub added_by_user: String,
    pub is_nsfw: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, FromRow)]
pub struct CreatePicture {
    pub name: String,
    pub guild_id: String,
    pub url: String,
    pub added_by_user: String,
    pub is_nsfw: bool,
}

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
