use std::thread;
use std::time::Duration;

use thirtyfour::prelude::{
    ChromiumLikeCapabilities, DesiredCapabilities, WebDriver, WebDriverResult,
};
use tokio::process::{Child, Command};

use crate::prelude::{Error, Result};

#[derive(Debug)]
pub struct DualWebDriver {
    child_process: Child,
    pub main_driver: WebDriver,
}

impl DualWebDriver {
    pub async fn new(port_number: u16) -> Result<Self> {
        let child_process = start_background(port_number).await?;
        let main_driver = start_main(port_number)
            .await
            .map_err(|e| Error::WebDriverInternal(e.to_string()))?;

        Ok(DualWebDriver {
            child_process,
            main_driver,
        })
    }

    pub async fn stop(self) -> Result<()> {
        stop_main(self.main_driver)
            .await
            .map_err(|e| Error::WebDriverInternal(e.to_string()))?;
        stop_background(self.child_process).await?;

        Ok(())
    }
}

async fn start_background(port_number: u16) -> Result<Child> {
    let mut child_process = Command::new("chromedriver")
        .arg(format!("--port={port_number}"))
        .spawn()?;
    eprintln!("Background webdriver starting...");
    thread::sleep(Duration::from_secs(1));

    match child_process.try_wait() {
        Ok(Some(_)) => {
            eprintln!("Child process already exited");
            Err(Error::WebDriverChild)
        }
        Err(e) => {
            eprintln!("Error attempting to wait: {e}");
            Err(Error::WebDriverChild)
        }
        Ok(None) => Ok(child_process),
    }
}

async fn start_main(port_number: u16) -> WebDriverResult<WebDriver> {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_arg("--incognito")?;
    caps.add_arg("--no-sandbox")?;
    caps.add_arg("--disable-dev-shm-usage")?;
    caps.add_arg("--disable-gpu")?;
    caps.add_arg("--headless")?;

    let server_url = format!("http://127.0.0.1:{port_number}");
    let driver = WebDriver::new(server_url, caps).await?;
    eprintln!("Main webdriver starting...");
    Ok(driver)
}

async fn stop_main(main_driver: WebDriver) -> WebDriverResult<()> {
    main_driver.quit().await?;
    eprintln!("Main webdriver stopped successfully");
    Ok(())
}

async fn stop_background(mut child_process: Child) -> Result<()> {
    child_process.kill().await?;
    eprintln!("Background webdriver process stopped successfully");
    Ok(())
}
