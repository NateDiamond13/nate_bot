use chrono::NaiveDateTime;
use sqlx::{query, query_as};

use crate::prelude::{DbExecutor, Result};

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

pub async fn get<'a>(
    dbx: impl DbExecutor<'a>,
    source_site: impl Into<String>,
) -> Option<AuthToken> {
    query_as!(
        AuthToken,
        "SELECT *
        FROM auth_tokens
        WHERE source_site = $1;",
        source_site.into()
    )
    .fetch_one(dbx)
    .await
    .ok()
}

pub async fn insert<'a>(
    dbx: impl DbExecutor<'a>,
    create_auth_token: &CreateAuthToken,
) -> Result<bool> {
    let insert_result = query!(
        "INSERT INTO auth_tokens (source_site, access_token, token_type, expires_at, refresh_token,
            scope)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (source_site)
            DO UPDATE SET
                access_token = EXCLUDED.access_token,
                token_type = EXCLUDED.token_type,
                expires_at = EXCLUDED.expires_at,
                refresh_token = EXCLUDED.refresh_token,
                scope = EXCLUDED.scope,
                updated_at = NOW();",
        create_auth_token.source_site,
        create_auth_token.access_token,
        create_auth_token.token_type,
        create_auth_token.expires_at,
        create_auth_token.refresh_token,
        create_auth_token.scope,
    )
    .execute(dbx)
    .await?;

    Ok(insert_result.rows_affected() > 0)
}
