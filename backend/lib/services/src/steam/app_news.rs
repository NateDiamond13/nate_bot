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

#[derive(Clone, Debug, Deserialize)]
pub struct SteamAppNewsItem {
    pub gid: String,
    pub title: String,
    pub url: String,
    pub contents: String,
    pub date: i64,
    pub feed_type: u16,
    pub appid: u32,
}

pub async fn get_app_news(
    client: &SteamClient,
    app_id: impl Into<String>,
    count: usize,
) -> Result<Vec<SteamAppNewsItem>> {
    let app_id = app_id.into();
    let response = client
        .request_client
        .inner_client()
        .get("https://api.steampowered.com/ISteamNews/GetNewsForApp/v0002")
        .header("Accept", "application/json; charset=utf-8")
        .query(&[
            ("format", "json"),
            ("appid", &app_id),
            ("count", &count.to_string()),
        ])
        .send()
        .await?;

    let result_json = response.json::<SteamAppNewsResponse>().await?;

    Ok(result_json.appnews.newsitems)
}
