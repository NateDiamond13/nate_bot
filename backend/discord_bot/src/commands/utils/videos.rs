use crate::prelude::{CommandData, Error, Result};

use rusty_ytdl::search::{SearchResult, YouTube};
use rusty_ytdl::{Video, VideoOptions, VideoQuality, VideoSearchOptions};
use songbird::input::{Compose, HttpRequest, Input, YoutubeDl};
use url::Url;

pub struct VideoDetails {
    pub input: Input,
    pub num_seconds: u64,
    pub source_url: String,
}

pub async fn get_video_details(url_or_search: &str, data: &CommandData) -> Result<VideoDetails> {
    let url_or_search = url_or_search.trim().to_string();
    if is_valid_url(&url_or_search) {
        if is_valid_youtube_url(&url_or_search) {
            // If it's a valid YouTube url, find it directly with rusty_ytdl
            get_youtube_video_by_url(&url_or_search, data).await
        } else {
            // If it's a valid non-YouTube url, find it with YoutubeDl
            let mut src = YoutubeDl::new(data.http_client.clone(), url_or_search);
            let metadata = src.aux_metadata().await?;

            Ok(VideoDetails {
                input: src.into(),
                num_seconds: metadata.duration.ok_or(Error::VideoDetailParse)?.as_secs(),
                source_url: metadata.source_url.ok_or(Error::VideoDetailParse)?,
            })
        }
    } else {
        // Otherwise search for it with rusty_ytdl
        let url = get_url_by_search(&url_or_search).await?;
        get_youtube_video_by_url(&url, data).await
    }
}

async fn get_youtube_video_by_url(url: &str, data: &CommandData) -> Result<VideoDetails> {
    let video_options = VideoOptions {
        quality: VideoQuality::Lowest,
        filter: VideoSearchOptions::Audio,
        ..Default::default()
    };
    let video = Video::new_with_options(url, video_options.clone())?;

    let info = video.get_info().await?;
    let format = rusty_ytdl::choose_format(info.formats.as_slice(), &video_options)?;
    let req = HttpRequest::new(data.http_client.clone(), format.url);

    Ok(VideoDetails {
        input: req.into(),
        num_seconds: info.video_details.length_seconds.parse::<u64>()?,
        source_url: info.video_details.video_url,
    })
}

async fn get_url_by_search(search: &str) -> Result<String> {
    let youtube = YouTube::new()?;
    match youtube.search_one(search, None).await? {
        Some(SearchResult::Video(video)) => Ok(video.url),
        _ => Err(Error::VideoNotFound),
    }
}

fn is_valid_youtube_url(url: &str) -> bool {
    rusty_ytdl::get_video_id(url).is_some()
}

fn is_valid_url(url: &str) -> bool {
    Url::parse(url).is_ok()
}
