use crate::prelude::{Error, Result};

use chrono::{Duration, Utc};
use database::{
    auth_tokens::{self, AuthToken, CreateAuthToken},
    PgPool,
};
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::{policies::ExponentialBackoff, RetryTransientMiddleware};
use serde::Deserialize;
use utils::EnvVariables;

const EXPIRY_GRACE_SECONDS: i64 = 20;

#[derive(Debug, Deserialize)]
struct SoundcloudTokenResult {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub scope: String,
}

#[derive(Debug, Deserialize)]
struct SearchResult {
    pub collection: Vec<SearchTrack>,
}

#[derive(Debug, Deserialize)]
struct SearchTrack {
    pub permalink_url: String,
}

#[derive(Debug)]
pub struct SoundcloudClient {
    mid_client: ClientWithMiddleware,
}

impl SoundcloudClient {
    pub fn new() -> Self {
        let mid_client = ClientBuilder::new(reqwest::Client::new())
            .with(RetryTransientMiddleware::new_with_policy(
                ExponentialBackoff::builder().build_with_max_retries(5),
            ))
            .build();
        Self { mid_client }
    }

    pub async fn search_for_track_url(
        &self,
        search_str: &str,
        conn: &PgPool,
        env_vars: &EnvVariables,
    ) -> Result<Option<String>> {
        let auth_token = get_token(&self.mid_client, &conn, &env_vars).await?;
        find_track_url(&search_str, &self.mid_client, &auth_token).await
    }
}

async fn find_track_url(
    search_str: &str,
    mid_client: &ClientWithMiddleware,
    auth_token: &AuthToken,
) -> Result<Option<String>> {
    println!("Searching Soundcloud for \"{}\"", &search_str);
    let response = mid_client
        .get("https://api.soundcloud.com/tracks")
        .header("Accept", "application/json; charset=utf-8")
        .header(
            "Authorization",
            format!("Bearer {}", &auth_token.access_token),
        )
        .query(&[
            ("q", search_str),
            ("access", "playable"),
            ("limit", "1"),
            ("linked_partitioning", "true"),
        ])
        .send()
        .await?;

    let result = response.json::<SearchResult>().await?;
    Ok(result.collection.get(0).map(|e| e.permalink_url.clone()))
}

async fn get_token(
    mid_client: &ClientWithMiddleware,
    conn: &PgPool,
    env_vars: &EnvVariables,
) -> Result<AuthToken> {
    let source_site = "soundcloud.com".to_string();
    let current_token = auth_tokens::get(conn, &source_site).await;

    let new_sc_token;
    if let Some(token) = current_token {
        // Check if token has not expired
        if Utc::now().naive_utc() < token.expires_at - Duration::seconds(EXPIRY_GRACE_SECONDS) {
            return Ok(token);
        }
        // If it's expired, refresh the token
        new_sc_token = refresh_token(&token, &mid_client, &env_vars).await?;
    } else {
        // If the token doesn't exist, fetch a new one
        new_sc_token = fetch_new_token(&mid_client, &env_vars).await?
    }

    let expires_at = Utc::now().naive_utc() + Duration::seconds(new_sc_token.expires_in as i64);
    let create_token = CreateAuthToken {
        source_site: source_site.clone(),
        access_token: new_sc_token.access_token,
        token_type: new_sc_token.token_type,
        expires_at,
        refresh_token: new_sc_token.refresh_token,
        scope: new_sc_token.scope,
    };
    auth_tokens::insert(conn, &create_token).await?;
    let new_token = auth_tokens::get(conn, &source_site)
        .await
        .ok_or(Error::MissingAuthToken(source_site))?;
    Ok(new_token)
}

async fn fetch_new_token(
    mid_client: &ClientWithMiddleware,
    env_vars: &EnvVariables,
) -> Result<SoundcloudTokenResult> {
    println!("Fetching new Soundcloud token");
    let response = mid_client
        .post("https://secure.soundcloud.com/oauth/token")
        .header("Accept", "application/json; charset=utf-8")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .basic_auth(
            &env_vars.soundcloud_client_id,
            Some(&env_vars.soundcloud_client_secret),
        )
        .body("grant_type=client_credentials")
        .send()
        .await?;
    let result = response.json::<SoundcloudTokenResult>().await?;
    Ok(result)
}

async fn refresh_token(
    auth_token: &AuthToken,
    mid_client: &ClientWithMiddleware,
    env_vars: &EnvVariables,
) -> Result<SoundcloudTokenResult> {
    println!("Refreshing Soundcloud token");
    let response = mid_client
        .post("https://secure.soundcloud.com/oauth/token")
        .header("Accept", "application/json; charset=utf-8")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .form(&[
            ("grant_type", "refresh_token"),
            ("client_id", &env_vars.soundcloud_client_id),
            ("client_secret", &env_vars.soundcloud_client_secret),
            ("refresh_token", &auth_token.refresh_token),
        ])
        .send()
        .await?;
    let result = response.json::<SoundcloudTokenResult>().await?;
    Ok(result)
}
