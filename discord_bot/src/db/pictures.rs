use crate::prelude::Result;

use chrono::NaiveDateTime;
use sqlx::{query, query_as, FromRow, PgPool};

#[allow(dead_code)]
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

pub async fn get(
    conn: &PgPool,
    name: impl Into<String>,
    guild_id: impl Into<String>,
) -> Option<Picture> {
    query_as!(
        Picture,
        "SELECT * FROM pictures
        WHERE name = $1 AND guild_id = $2;",
        name.into(),
        guild_id.into()
    )
    .fetch_one(conn)
    .await
    .ok()
}

pub async fn get_all(conn: &PgPool, guild_id: impl Into<String>) -> Option<Vec<Picture>> {
    query_as!(
        Picture,
        "SELECT * FROM pictures
        WHERE guild_id = $1
        ORDER BY name;",
        guild_id.into()
    )
    .fetch_all(conn)
    .await
    .ok()
}

pub async fn get_random(
    conn: &PgPool,
    guild_id: impl Into<String>,
    is_nsfw: Option<bool>,
) -> Option<Picture> {
    match is_nsfw {
        Some(flag_nsfw) => query_as!(
            Picture,
            "SELECT * FROM pictures
            WHERE guild_id = $1 AND is_nsfw = $2
            ORDER BY random() LIMIT 1;",
            guild_id.into(),
            flag_nsfw
        )
        .fetch_one(conn)
        .await
        .ok(),
        None => query_as!(
            Picture,
            "SELECT * FROM pictures
            WHERE guild_id = $1
            ORDER BY random() LIMIT 1;",
            guild_id.into()
        )
        .fetch_one(conn)
        .await
        .ok(),
    }
}

pub async fn insert(conn: &PgPool, create_pic: &CreatePicture) -> Result<()> {
    query(
        "INSERT INTO pictures (name, guild_id, url, added_by_user, is_nsfw)
        VALUES ($1, $2, $3, $4, $5);",
    )
    .bind(&create_pic.name)
    .bind(&create_pic.guild_id)
    .bind(&create_pic.url)
    .bind(&create_pic.added_by_user)
    .bind(create_pic.is_nsfw)
    .execute(conn)
    .await?;
    Ok(())
}

pub async fn remove(
    conn: &PgPool,
    name: impl Into<String>,
    guild_id: impl Into<String>,
) -> Result<()> {
    query(
        "DELETE FROM pictures
        WHERE name = $1 AND guild_id = $2;",
    )
    .bind(name.into())
    .bind(guild_id.into())
    .execute(conn)
    .await?;
    Ok(())
}
