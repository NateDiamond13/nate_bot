use reqwest::IntoUrl;
use reqwest_middleware::{ClientBuilder, ClientWithMiddleware, RequestBuilder};
use reqwest_retry::RetryTransientMiddleware;
use reqwest_retry::policies::ExponentialBackoff;

const MAX_RETRIES: u32 = 5;

/// Client for sendings requests to external APIs
#[derive(Clone, Debug)]
pub struct RequestClient {
    mid_client: ClientWithMiddleware,
}

impl Default for RequestClient {
    fn default() -> Self {
        Self::new()
    }
}

impl RequestClient {
    /// Create a new [`RequestClient`] that will retry failed requests with exponential backoff
    pub fn new() -> Self {
        let mid_client = ClientBuilder::new(reqwest::Client::new())
            .with(RetryTransientMiddleware::new_with_policy(
                ExponentialBackoff::builder().build_with_max_retries(MAX_RETRIES),
            ))
            .build();

        Self { mid_client }
    }

    /// Make a `GET` request to a URL
    pub fn get(&self, url: impl IntoUrl) -> RequestBuilder {
        self.mid_client.get(url)
    }
}
