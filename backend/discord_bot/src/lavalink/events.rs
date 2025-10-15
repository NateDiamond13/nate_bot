use lavalink_rs::client::LavalinkClient;
use lavalink_rs::hook;
use lavalink_rs::model::events;
use serde_json::Value;

#[hook]
pub async fn raw_event(_: LavalinkClient, session_id: String, event: &Value) {
    if event["op"].as_str() == Some("event") || event["op"].as_str() == Some("playerUpdate") {
        log::info!("{:?} -> {:?}", session_id, event);
    }
}

#[hook]
pub async fn ready_event(client: LavalinkClient, session_id: String, event: &events::Ready) {
    if let Err(err) = client.delete_all_player_contexts().await {
        log::error!("Unable to delete player contexts: {err:#?}");
    }
    log::info!("{:?} -> {:?}", session_id, event);
}

#[hook]
pub async fn track_start(client: LavalinkClient, _session_id: String, event: &events::TrackStart) {
    let Some(_player_context) = client.get_player_context(event.guild_id) else {
        return;
    };

    let msg = {
        let track = &event.track;
        let Some(user_data) = &track.user_data else {
            return;
        };
        let Some(requester_id) = user_data.get("requester_id") else {
            return;
        };

        if let Some(uri) = &track.info.uri {
            format!(
                "Now playing: [{} - {}](<{}>) | Requested by <@!{}>",
                track.info.author, track.info.title, uri, requester_id
            )
        } else {
            format!(
                "Now playing: {} - {} | Requested by <@!{}>",
                track.info.author, track.info.title, requester_id
            )
        }
    };

    log::info!("{msg}");
}

#[hook]
pub async fn track_end(client: LavalinkClient, _session_id: String, event: &events::TrackEnd) {
    let Some(player_context) = client.get_player_context(event.guild_id) else {
        return;
    };

    let queue_count = player_context.get_queue().get_count().await;
    log::info!("Songs left in queue: {queue_count:?}");
}
