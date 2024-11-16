use crate::prelude::{Error, Result};

use chrono::{DateTime, NaiveDateTime};
use database::{
    patch_notes::{self, CreatePatchNotes},
    patch_notes_subscriptions,
};
use regex::Regex;
use thirtyfour::prelude::{By, WebDriver, WebDriverResult, WebElement};

#[derive(Debug)]
struct ChangelogThreadMetadata {
    pub title: String,
    pub patch_id: String,
    pub ref_link: String,
    pub timestamp: NaiveDateTime,
}

pub async fn update_latest(driver: &WebDriver, database_url: String) -> Result<bool> {
    let Some(latest_patch_notes) = get_latest_patch_notes(driver)
        .await
        .map_err(|e| Error::WebDriverInternal(e.to_string()))?
    else {
        return Ok(false);
    };

    let conn = database::get_connection_pool(&database_url).await?;
    let insert_result = patch_notes::insert(&conn, &latest_patch_notes).await?;

    // If there's a new patch
    if insert_result == 1 {
        let target_game = "deadlock";
        println!("New patch found for game: {target_game}");

        // Send alerts to subscribed channels
        let Some(subs) = patch_notes_subscriptions::get_all_for_game(&conn, target_game).await
        else {
            return Ok(false);
        };
        let Some(latest_patch) = patch_notes::get_latest(&conn, target_game).await else {
            return Ok(false);
        };
        webhooks::patch_notes::send_all_alerts(&latest_patch, &subs).await?;
    }

    Ok(true)
}

async fn get_latest_patch_notes(driver: &WebDriver) -> WebDriverResult<Option<CreatePatchNotes>> {
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
        return Ok(None);
    };

    // Navigate to patch thread page
    let thread_url = format!("https://forums.playdeadlock.com{}", metadata.ref_link);
    driver.goto(&thread_url).await?;

    // Get patch notes
    let block_wrapper = driver.find(By::ClassName("bbWrapper")).await?;
    let content_block = block_wrapper.inner_html().await?;

    Ok(Some(CreatePatchNotes {
        target_game: "deadlock".to_string(),
        patch_id: metadata.patch_id,
        link: thread_url,
        title: metadata.title,
        content: content_block,
        posted_at: metadata.timestamp,
    }))
}

async fn get_thread_metadata(
    thread_elem: WebElement,
) -> WebDriverResult<Option<ChangelogThreadMetadata>> {
    let title_anchor = thread_elem
        .find(By::ClassName("structItem-title"))
        .await?
        .find(By::Tag("a"))
        .await?;
    let title = title_anchor.inner_html().await?.trim().to_string();

    let Some(ref_link) = title_anchor.attr("href").await? else {
        eprintln!("-- Missing thread metadata: ref_link");
        return Ok(None);
    };
    let Some(patch_id) = parse_id(&ref_link) else {
        eprintln!("-- Missing thread metadata: patch_id");
        return Ok(None);
    };

    let timestamp_str = thread_elem
        .find(By::ClassName("structItem-startDate"))
        .await?
        .find(By::ClassName("u-dt"))
        .await?
        .attr("datetime")
        .await?;
    let Some(timestamp) = parse_timestamp(timestamp_str) else {
        eprintln!("-- Missing thread metadata: timestamp");
        return Ok(None);
    };

    Ok(Some(ChangelogThreadMetadata {
        title,
        patch_id,
        ref_link,
        timestamp,
    }))
}

fn parse_timestamp(timestamp: Option<String>) -> Option<NaiveDateTime> {
    let time_format = "%Y-%m-%dT%H:%M:%S%z";
    Some(
        DateTime::parse_from_str(timestamp?.as_str(), time_format)
            .ok()?
            .naive_utc(),
    )
}

fn parse_id(ref_link: &str) -> Option<String> {
    let re = Regex::new(r"\.(.+)\/$").ok()?;
    let (_, [patch_id]) = re.captures(ref_link)?.extract();
    Some(patch_id.to_string())
}
