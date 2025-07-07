mod deadlock;
mod webdriver;

use webdriver::DualWebDriver;

use crate::prelude::Result;

pub async fn execute_job() -> Result<()> {
    log::info!("Executing test scraper job task");

    let env_vars = utils::get_config();
    let database_url = env_vars.database_url;
    let port_number = env_vars.webdriver_port;
    let driver = DualWebDriver::new(port_number).await?;

    match deadlock::update_latest(&driver.main_driver, database_url).await {
        Ok(_) => log::info!("Deadlock update finished successfully"),
        Err(err) => log::error!("Deadlock update finished with error: {err}"),
    };

    driver.stop().await?;

    Ok(())
}
