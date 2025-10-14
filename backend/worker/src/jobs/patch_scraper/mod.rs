mod deadlock;
mod webdriver;

use database::DbPool;
use utils::EnvVariables;
use webdriver::DualWebDriver;

use crate::prelude::Result;

/// Run the Patch Scraper job
pub async fn patch_scraper_job(db_pool: &DbPool, env_vars: &EnvVariables) -> Result<bool> {
    let port_number = env_vars.webdriver_port;
    let web_driver = DualWebDriver::new(port_number).await?;

    // Update patch notes for Deadlock
    let deadlock_result = deadlock::update_latest_notes(db_pool, &web_driver.main_driver).await;
    match &deadlock_result {
        Ok(res) => log::info!("Deadlock update finished successfully with result: {res}"),
        Err(err) => log::error!("Deadlock update finished with error: {err}"),
    };

    web_driver.stop().await?;

    Ok(deadlock_result.is_ok())
}

#[cfg(test)]
mod tests {
    use database::DbPool;
    use test_log::test;

    use crate::jobs::patch_scraper::patch_scraper_job;
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
}
