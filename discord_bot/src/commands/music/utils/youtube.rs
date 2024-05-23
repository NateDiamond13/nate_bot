use crate::prelude::{CommandData, Result};

use rusty_ytdl::{Video, VideoInfo, VideoOptions, VideoQuality, VideoSearchOptions};
use songbird::input::{HttpRequest, Input};

pub struct VideoInput {
    pub input: Input,
    pub info: VideoInfo,
}

pub async fn download_video(url: impl Into<String>, data: &CommandData) -> Result<VideoInput> {
    let video_options = VideoOptions {
        quality: VideoQuality::Lowest,
        filter: VideoSearchOptions::Audio,
        ..Default::default()
    };
    let video = Video::new_with_options(url, video_options.clone())?;

    let info = video.get_info().await?;
    let format = rusty_ytdl::choose_format(info.formats.as_slice(), &video_options)?;

    let req = HttpRequest::new(data.http_client.clone(), format.url);
    Ok(VideoInput {
        input: req.into(),
        info,
    })
}

pub fn is_valid_url(url: &str) -> bool {
    rusty_ytdl::get_video_id(url).is_some()
}
