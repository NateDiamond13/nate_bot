use lavalink_rs::client::LavalinkClient;
use lavalink_rs::model::UserId;
use lavalink_rs::model::events::Events;
use lavalink_rs::node::NodeBuilder;
use lavalink_rs::prelude::NodeDistributionStrategy;

pub mod commands;
mod events;

pub async fn get_temp_client(
    hostname: impl Into<String>,
    password: impl Into<String>,
) -> LavalinkClient {
    get_client(hostname, password, 0, "").await
}

pub async fn get_client(
    hostname: impl Into<String>,
    password: impl Into<String>,
    user_id: u64,
    session_id: impl Into<String>,
) -> LavalinkClient {
    let client_events = Events {
        raw: Some(events::raw_event),
        ready: Some(events::ready_event),
        ..Default::default()
    };

    let node_local = NodeBuilder {
        hostname: hostname.into(),
        password: password.into(),
        is_ssl: false,
        events: Events::default(),
        user_id: UserId(user_id),
        session_id: Some(session_id.into()),
    };

    LavalinkClient::new(
        client_events,
        vec![node_local],
        NodeDistributionStrategy::round_robin(),
    )
    .await
}
