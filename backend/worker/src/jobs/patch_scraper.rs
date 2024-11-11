use std::{thread, time::Duration};

use crate::prelude::{Error, Result};

use thirtyfour::prelude::*;
use tokio::process::{Child, Command};

pub async fn execute_job() -> Result<()> {
    println!("Executing test scraper job task");

    let port_number = utils::get_env_variables().webdriver_port;
    let child_proc = start_webdriver(port_number, 1).await?;

    match inner_job(port_number).await {
        Ok(_) => println!("Job finished successfully"),
        Err(e) => println!("Job finished with error: {e}"),
    };

    stop_webdriver(child_proc).await?;
    Ok(())
}

async fn start_webdriver(port: u16, start_delay_secs: u64) -> Result<Child> {
    let mut child_proc = Command::new("chromedriver")
        .arg(format!("--port={port}"))
        .spawn()?;
    eprintln!("WebDriver starting...");
    thread::sleep(Duration::from_secs(start_delay_secs));

    match child_proc.try_wait() {
        Ok(Some(_)) => {
            eprintln!("Child process already exited");
            Err(Error::WebDriverChild)
        }
        Err(e) => {
            eprintln!("Error attempting to wait: {e}");
            Err(Error::WebDriverChild)
        }
        Ok(None) => {
            eprintln!("Continuing...");
            Ok(child_proc)
        }
    }
}

async fn stop_webdriver(mut child: Child) -> Result<()> {
    child.kill().await?;
    eprintln!("WebDriver process killed successfully");
    Ok(())
}

#[derive(Debug, Clone)]
struct PatchThreadMetadata {
    pub title: String,
    pub link: String,
    pub author: String,
}

async fn get_thread_metadata(
    thread_elem: WebElement,
) -> WebDriverResult<Option<PatchThreadMetadata>> {
    let title_anchor = thread_elem
        .find(By::ClassName("structItem-title"))
        .await?
        .find(By::Tag("a"))
        .await?;
    let title = title_anchor.inner_html().await?.trim().to_string();

    let Some(link) = title_anchor.attr("href").await? else {
        eprintln!("-- Missing thread metadata: link");
        return Ok(None);
    };

    let Some(author) = thread_elem.attr("data-author").await? else {
        eprintln!("-- Missing thread metadata: author");
        return Ok(None);
    };

    Ok(Some(PatchThreadMetadata {
        title,
        link,
        author,
    }))
}

async fn inner_job(port: u16) -> WebDriverResult<()> {
    let mut caps = DesiredCapabilities::chrome();
    caps.add_arg("--no-sandbox")?;
    caps.add_arg("--disable-dev-shm-usage")?;
    caps.add_arg("--disable-gpu")?;
    caps.add_arg("--headless")?;

    let server_url = format!("http://127.0.0.1:{port}");
    let driver = WebDriver::new(server_url, caps).await?;

    // Navigate to patch webpage
    driver
        .goto("https://forums.playdeadlock.com/forums/changelog.10/")
        .await?;

    // Find most recent patch thread
    let thread_list = driver.find(By::ClassName("js-threadList")).await?;
    let thread = thread_list
        .find(By::ClassName("structItem--thread"))
        .await?;

    // Parse thread post for metadata
    let Some(metadata) = get_thread_metadata(thread).await? else {
        driver.quit().await?;
        return Ok(());
    };
    println!("{}", metadata.author);
    println!("{}", metadata.title);
    println!("{}", metadata.link);

    // Navigate to patch thread page
    let thread_url = format!("https://forums.playdeadlock.com{}", metadata.link);
    driver.goto(thread_url).await?;

    // Get patch notes
    let block_wrapper = driver.find(By::ClassName("bbWrapper")).await?;
    let notes_block = block_wrapper.inner_html().await?;
    println!("{:?}", notes_block);

    driver.quit().await?;
    Ok(())
}
