mod deadlock;
mod webdriver;

use webdriver::DualWebDriver;

use crate::prelude::Result;

pub async fn execute_job() -> Result<()> {
    println!("Executing test scraper job task");

    let env_vars = utils::get_env_variables();
    let database_url = env_vars.database_url;
    let port_number = env_vars.webdriver_port;
    let driver = DualWebDriver::new(port_number).await?;

    match deadlock::update_latest(&driver.main_driver, database_url).await {
        Ok(_) => println!("Deadlock update finished successfully"),
        Err(e) => println!("Deadlock update finished with error: {e}"),
    };

    driver.stop().await?;

    Ok(())
}
