use crate::prelude::Result;

use chrono::NaiveDateTime;
use sqlx::{query, query_as, PgPool};

#[derive(Debug)]
pub struct AuthToken {
    pub source_site: String,
    pub access_token: String,
    pub token_type: String,
    pub expires_at: NaiveDateTime,
    pub refresh_token: String,
    pub scope: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug)]
pub struct CreateAuthToken {
    pub source_site: String,
    pub access_token: String,
    pub token_type: String,
    pub expires_at: NaiveDateTime,
    pub refresh_token: String,
    pub scope: String,
}

pub async fn get(conn: &PgPool, source_site: impl Into<String>) -> Option<AuthToken> {
    query_as!(
        AuthToken,
        "SELECT * FROM auth_tokens
        WHERE source_site = $1;",
        source_site.into()
    )
    .fetch_one(conn)
    .await
    .ok()
}

pub async fn insert(conn: &PgPool, create_auth_token: &CreateAuthToken) -> Result<()> {
    query!(
        "INSERT INTO auth_tokens
            (source_site, access_token, token_type, expires_at, refresh_token, scope)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (source_site) DO UPDATE
        SET access_token = $2, token_type = $3, expires_at = $4, refresh_token = $5,
            scope = $6, updated_at = NOW();",
        create_auth_token.source_site,
        create_auth_token.access_token,
        create_auth_token.token_type,
        create_auth_token.expires_at,
        create_auth_token.refresh_token,
        create_auth_token.scope,
    )
    .execute(conn)
    .await?;
    Ok(())
}
