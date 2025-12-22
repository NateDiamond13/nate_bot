use chrono::DateTime;
use database::DbTransaction;
use database::patch_notes::{self, CreatePatchNotes, SteamPatchMetadata};
use services::{SteamAppNewsItem, SteamClient};

use crate::jobs::patch_scraper::bbcode;
use crate::prelude::Result;

const MAX_NEWS_COUNT: usize = 5;

pub async fn update_latest_news(
    tx: &mut DbTransaction,
    steam_client: &SteamClient,
) -> Result<Vec<String>> {
    // Get latest steam patch info from db
    let current_patches = patch_notes::get_latest_steam_patches(tx.as_mut()).await?;

    // Create a vec to store successfully updated target games
    let mut updated_games = vec![];

    // For each app id, fetch latest news results from Steam API
    for cur_patch in current_patches {
        let app_news = get_latest_app_news(&cur_patch, steam_client, MAX_NEWS_COUNT).await;

        if let Some(news_item) = app_news
            && let Some(create_patch_notes) = convert_patch_notes(&cur_patch, news_item)
        {
            patch_notes::insert(tx.as_mut(), &create_patch_notes).await?;
            updated_games.push(create_patch_notes.target_game.clone());
        }
    }

    Ok(updated_games)
}

async fn get_latest_app_news(
    patch_meta: &SteamPatchMetadata,
    steam_client: &SteamClient,
    max_result_count: usize,
) -> Option<SteamAppNewsItem> {
    let news_result = steam_client
        .get_app_news(&patch_meta.steam_app_id, max_result_count)
        .await
        .ok()?;

    match patch_meta.latest_posted_at {
        Some(latest_posted_at) => {
            let latest_timestamp = latest_posted_at.and_utc().timestamp();
            for news_item in news_result {
                // Check if the news is older than the latest in the db
                if latest_timestamp >= news_item.date {
                    return None;
                }

                // If item is from the patch notes feed (feed_type = 1)
                if news_item.feed_type == 1 {
                    return Some(news_item);
                }
            }
            None
        }
        None => {
            for news_item in news_result {
                // If item is from the patch notes feed (feed_type = 1)
                if news_item.feed_type == 1 {
                    return Some(news_item);
                }
            }
            None
        }
    }
}

fn convert_patch_notes(
    patch_metadata: &SteamPatchMetadata,
    news_item: SteamAppNewsItem,
) -> Option<CreatePatchNotes> {
    let posted_at = DateTime::from_timestamp_secs(news_item.date)?.naive_utc();
    let content = bbcode::convert_to_html(&news_item.contents);

    Some(CreatePatchNotes {
        target_game: patch_metadata.target_game.clone(),
        patch_id: news_item.gid,
        link: news_item.url,
        title: news_item.title,
        content,
        posted_at,
    })
}

#[cfg(test)]
mod tests {
    use database::DbPool;
    use services::SteamClient;
    use test_log::test;

    use crate::jobs::patch_scraper::steam_apps::update_latest_news;
    use crate::prelude::Result;

    #[ignore]
    #[test(tokio::test)]
    async fn test_update_latest_news() -> Result<()> {
        let env_vars = utils::get_config();
        let db_pool = DbPool::new(&env_vars.database_url).await?;
        let steam_client = SteamClient::default();

        let mut tx = db_pool.begin_transaction().await?;

        let result = update_latest_news(&mut tx, &steam_client).await?;
        log::info!("{result:#?}");

        database::commit_transaction(tx).await?;

        Ok(())
    }
}
