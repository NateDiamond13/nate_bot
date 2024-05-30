use crate::prelude::{CommandData, Error, Result};

use rusty_ytdl::{Video, VideoOptions, VideoQuality, VideoSearchOptions};
use songbird::input::{Compose, HttpRequest, Input, YoutubeDl};

pub struct VideoDetails {
    pub input: Input,
    pub num_seconds: u64,
    pub source_url: String,
}

pub async fn get_video_details(url: impl Into<String>, data: &CommandData) -> Result<VideoDetails> {
    let url = url.into();
    if is_valid_youtube_url(&url) {
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
    } else {
        let mut src = YoutubeDl::new(data.http_client.clone(), url);
        let metadata = src.aux_metadata().await?;

        Ok(VideoDetails {
            input: src.into(),
            num_seconds: metadata.duration.ok_or(Error::VideoDetailParse)?.as_secs(),
            source_url: metadata.source_url.ok_or(Error::VideoDetailParse)?,
        })
    }
}

fn is_valid_youtube_url(url: &str) -> bool {
    rusty_ytdl::get_video_id(url).is_some()
}
