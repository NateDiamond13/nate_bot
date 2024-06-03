use crate::prelude::Result;

use sqlx::{query, query_as, FromRow, PgPool};

#[derive(Debug, FromRow)]
pub struct KeyValPair {
    pub key: String,
    pub value: String,
}

pub async fn get(conn: &PgPool, key: impl Into<String>) -> Option<KeyValPair> {
    query_as!(
        KeyValPair,
        "SELECT * FROM store WHERE key = $1;",
        key.into()
    )
    .fetch_one(conn)
    .await
    .ok()
}

pub async fn get_all(conn: &PgPool) -> Option<Vec<KeyValPair>> {
    query_as!(KeyValPair, "SELECT * FROM store ORDER BY key;")
        .fetch_all(conn)
        .await
        .ok()
}

pub async fn insert(conn: &PgPool, pair: &KeyValPair) -> Result<()> {
    query("INSERT INTO store (key, value) VALUES ($1, $2);")
        .bind(&pair.key)
        .bind(&pair.value)
        .execute(conn)
        .await?;
    Ok(())
}
