use serde::Deserialize;

use crate::prelude::Result;
use crate::steam::SteamClient;

#[derive(Deserialize)]
struct SteamAppNewsResponse {
    appnews: SteamAppNews,
}

#[derive(Deserialize)]
struct SteamAppNews {
    newsitems: Vec<SteamAppNewsItem>,
}

/// Response item from the Steam App News API
#[derive(Clone, Debug, Deserialize)]
pub struct SteamAppNewsItem {
    /// Internal Steam ID of news item
    pub gid: String,
    /// Title of article
    pub title: String,
    /// URL link to article
    pub url: String,
    /// Contents of article
    pub contents: String,
    /// Date (in epoch seconds)
    pub date: i64,
    /// Feed type of item ("0" is default, "1" is patch notes, etc.)
    pub feed_type: u16,
    /// Steam App ID for requested game
    pub appid: u32,
}

/// Get the latest news for a given Steam App
pub async fn get_app_news(
    client: &SteamClient,
    app_id: impl Into<String>,
    count: usize,
) -> Result<Vec<SteamAppNewsItem>> {
    let response = client
        .request_client
        .get("https://api.steampowered.com/ISteamNews/GetNewsForApp/v0002")
        .header("Accept", "application/json; charset=utf-8")
        .query(&[
            ("format", "json"),
            ("appid", &app_id.into()),
            ("count", &count.to_string()),
        ])
        .send()
        .await?;

    let result_json = response.json::<SteamAppNewsResponse>().await?;

    Ok(result_json.appnews.newsitems)
}
