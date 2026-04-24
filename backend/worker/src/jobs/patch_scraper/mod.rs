mod bbcode;
mod deadlock;
mod steam_apps;
mod webdriver;

use database::{DbPool, DbTransaction, patch_notes, patch_notes_subscriptions};
use services::SteamClient;
use utils::EnvVariables;
use webdriver::DualWebDriver;

use crate::prelude::Result;

/// Run the Patch Scraper job
pub async fn patch_scraper_job(db_pool: &DbPool, env_vars: &EnvVariables) -> Result<bool> {
    let port_number = env_vars.webdriver_port;
    let web_driver = DualWebDriver::new(port_number).await?;

    // Keep track of games that have updated patch notes so that we can send alerts
    let mut alert_games: Vec<String> = vec![];

    // Start database transaction
    let mut tx = db_pool.begin_transaction().await?;

    // Update patch notes for Deadlock
    let deadlock_result = deadlock::update_latest_notes(&mut tx, &web_driver.main_driver).await;
    match deadlock_result {
        Ok(res) => {
            log::info!(
                "Deadlock update finished successfully with result: {}",
                res.is_some()
            );
            if let Some(target_game) = res {
                alert_games.push(target_game)
            }
        }
        Err(err) => log::error!("Deadlock update finished with error: {err}"),
    };

    web_driver.stop().await?;

    // Update patch notes for Steam Apps
    let steam_client = SteamClient::default();
    let steam_update_result = steam_apps::update_latest_news(&mut tx, &steam_client).await;
    match steam_update_result {
        Ok(res) => {
            log::info!(
                "Steam Apps update finished successfully for {} app(s)",
                res.len()
            );
            alert_games.extend(res);
        }
        Err(err) => log::error!("Steam Apps update finished with error: {err}"),
    }

    // Send alerts to subscribers for new updates
    let alert_result = alert_subscribers(&mut tx, alert_games).await?;

    // Commit transaction
    database::commit_transaction(tx).await?;

    Ok(alert_result)
}

async fn alert_subscribers(tx: &mut DbTransaction, alert_games: Vec<String>) -> Result<bool> {
    if alert_games.is_empty() {
        return Ok(false);
    }

    for target_game in alert_games {
        if let Some(subs) =
            patch_notes_subscriptions::get_all_for_game(tx.as_mut(), &target_game).await
            && !subs.is_empty()
            && let Some(latest_patch) = patch_notes::get_latest(tx.as_mut(), &target_game).await
        {
            log::info!(
                "Sending {} alert(s) for subscribed game: \"{}\"",
                subs.len(),
                latest_patch.target_game
            );
            webhooks::send_all_patch_alerts(&latest_patch, &subs).await?;
        }
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use database::DbPool;
    use test_log::test;

    use crate::jobs::patch_scraper::{alert_subscribers, patch_scraper_job};
    use crate::prelude::Result;

    #[ignore]
    #[test(tokio::test)]
    async fn test_patch_scraper_job() -> Result<()> {
        let env_vars = utils::get_config();
        let db_pool = DbPool::new(&env_vars.database_url).await?;

        let result = patch_scraper_job(&db_pool, &env_vars).await?;
        log::info!("Patch Scraper Job result: {result}");

        Ok(())
    }

    #[ignore]
    #[test(tokio::test)]
    async fn test_alert_subscribers() -> Result<()> {
        let env_vars = utils::get_config();
        let db_pool = DbPool::new(&env_vars.database_url).await?;

        let mut tx = db_pool.begin_transaction().await?;

        let alert_games = vec!["arc raiders".to_string()];
        let result = alert_subscribers(&mut tx, alert_games).await?;
        log::info!("{result:#?}");

        database::commit_transaction(tx).await?;

        Ok(())
    }
}
