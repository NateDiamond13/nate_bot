use reqwest_middleware::{ClientBuilder, ClientWithMiddleware};
use reqwest_retry::RetryTransientMiddleware;
use reqwest_retry::policies::ExponentialBackoff;

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
    pub fn new() -> Self {
        let mid_client = ClientBuilder::new(reqwest::Client::new())
            .with(RetryTransientMiddleware::new_with_policy(
                ExponentialBackoff::builder().build_with_max_retries(10),
            ))
            .build();

        Self { mid_client }
    }

    pub fn inner_client(&self) -> &ClientWithMiddleware {
        &self.mid_client
    }
}
