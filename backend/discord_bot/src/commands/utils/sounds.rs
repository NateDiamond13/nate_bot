use songbird::input::{Compose, Input, YoutubeDl};
use url::Url;

use crate::prelude::{CommandData, Error, Result};
use crate::services;

#[derive(Debug, PartialEq)]
enum SoundSource {
    Soundcloud,
    Youtube,
    Other,
}

pub struct SoundDetails {
    pub input: Input,
    pub num_seconds: u64,
    pub source_url: String,
}

pub async fn get_sound_details(
    url_or_search: &str,
    data: &CommandData,
) -> Result<Option<SoundDetails>> {
    let source_str = url_or_search.trim().to_string();
    match get_source(&source_str) {
        Some(SoundSource::Soundcloud) => get_download_details(source_str, data).await,
        Some(SoundSource::Youtube) => Ok(None),
        Some(SoundSource::Other) => get_download_details(source_str, data).await,
        None => {
            // Search in Soundcloud API
            match services::soundcloud::search_track(source_str, data).await? {
                Some(search_url) => get_download_details(search_url, data).await,
                None => Ok(None),
            }
        }
    }
}

fn get_source(url_or_search: &str) -> Option<SoundSource> {
    match Url::parse(url_or_search).ok()?.host_str() {
        Some("soundcloud.com") => Some(SoundSource::Soundcloud),
        Some("youtube.com") => Some(SoundSource::Youtube),
        Some(_) => Some(SoundSource::Other),
        _ => None,
    }
}

async fn get_download_details(
    url: impl Into<String>,
    data: &CommandData,
) -> Result<Option<SoundDetails>> {
    let mut src = YoutubeDl::new(data.http_client.clone(), url.into());
    let metadata = src.aux_metadata().await?;
    Ok(Some(SoundDetails {
        input: src.into(),
        num_seconds: metadata.duration.ok_or(Error::VideoDetailParse)?.as_secs(),
        source_url: metadata.source_url.ok_or(Error::VideoDetailParse)?,
    }))
}
