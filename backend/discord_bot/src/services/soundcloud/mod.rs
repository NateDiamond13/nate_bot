mod client;

use crate::prelude::{CommandData, Result};

pub async fn search_track(
    search_str: impl Into<String>,
    data: &CommandData,
) -> Result<Option<String>> {
    let search_str: String = search_str.into();
    client::SoundcloudClient::new()
        .search_for_track_url(&search_str, &data.pool, &data.env)
        .await
}
