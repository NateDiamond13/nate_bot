mod app_news;

use crate::client::RequestClient;
use crate::prelude::Result;
pub use crate::steam::app_news::SteamAppNewsItem;
use crate::steam::app_news::get_app_news;

/// [`RequestClient`] wrapper for handling requests to Steam API
#[derive(Clone, Debug)]
pub struct SteamClient {
    request_client: RequestClient,
}

impl Default for SteamClient {
    /// Create a new [`SteamClient`] from a default [`RequestClient`]
    fn default() -> Self {
        Self::new(RequestClient::default())
    }
}

impl SteamClient {
    /// Create a new [`SteamClient`] from a [`RequestClient`]
    pub fn new(request_client: RequestClient) -> Self {
        Self { request_client }
    }

    /// Get the latest `count` news items for a Steam App with the given `app_id`
    pub async fn get_app_news(
        &self,
        app_id: impl Into<String>,
        count: usize,
    ) -> Result<Vec<SteamAppNewsItem>> {
        get_app_news(self, app_id, count).await
    }
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use crate::prelude::Result;
    use crate::steam::SteamClient;

    #[ignore]
    #[test(tokio::test)]
    async fn test_get_app_news() -> Result<()> {
        let client = SteamClient::default();
        let result = client.get_app_news("1808500", 1).await?;
        log::info!("{result:#?}");

        Ok(())
    }
}
