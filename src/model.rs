//! Structs to represent the output of youtube-dl.
//! The definitions in this module are mostly auto-generated from the JSON output format of youtube-dl.

#![allow(missing_docs)]

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Chapter {
    pub end_time: Option<f64>,
    pub start_time: Option<f64>,
    pub title: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Comment {
    pub author: Option<String>,
    pub author_id: Option<String>,
    pub html: Option<String>,
    pub id: Option<String>,
    pub parent: Option<String>,
    pub text: Option<String>,
    pub timestamp: Option<f64>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Format {
    pub abr: Option<f64>,
    #[serde(default, deserialize_with = "parse_codec")]
    pub acodec: Option<String>,
    pub asr: Option<f64>,
    pub container: Option<String>,
    pub downloader_options: Option<BTreeMap<String, Value>>,
    pub ext: Option<String>,
    pub filesize: Option<f64>,
    pub filesize_approx: Option<f64>,
    pub format: Option<String>,
    pub format_id: Option<String>,
    pub format_note: Option<String>,
    pub fps: Option<f64>,
    pub fragment_base_url: Option<String>,
    pub fragments: Option<Vec<Fragment>>,
    pub height: Option<f64>,
    pub http_headers: Option<BTreeMap<String, Option<String>>>,
    pub language: Option<String>,
    pub language_preference: Option<i64>,
    pub manifest_url: Option<String>,
    pub no_resume: Option<bool>,
    pub player_url: Option<String>,
    pub preference: Option<Value>,
    pub protocol: Option<Protocol>,
    pub quality: Option<f64>,
    pub resolution: Option<String>,
    pub source_preference: Option<i64>,
    pub stretched_ratio: Option<f64>,
    pub tbr: Option<f64>,
    pub url: Option<String>,
    pub vbr: Option<f64>,
    #[serde(default, deserialize_with = "parse_codec")]
    pub vcodec: Option<String>,
    pub width: Option<f64>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Fragment {
    pub duration: Option<Value>,
    pub filesize: Option<i64>,
    pub path: Option<String>,
    pub url: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct JsonOutput {
    pub age_limit: Option<i64>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub album_type: Option<String>,
    pub alt_title: Option<String>,
    pub artist: Option<String>,
    pub automatic_captions: Option<BTreeMap<String, Vec<Subtitle>>>,
    pub average_rating: Option<Value>,
    pub categories: Option<Vec<Option<String>>>,
    pub channel: Option<String>,
    pub channel_id: Option<String>,
    pub channel_url: Option<String>,
    pub chapter: Option<String>,
    pub chapter_id: Option<String>,
    pub chapter_number: Option<String>,
    pub chapters: Option<Vec<Chapter>>,
    pub comment_count: Option<i64>,
    pub comments: Option<Vec<Comment>>,
    pub creator: Option<String>,
    pub description: Option<String>,
    pub disc_number: Option<i64>,
    pub dislike_count: Option<i64>,
    pub display_id: Option<String>,
    pub duration: Option<Value>,
    pub duration_string: Option<String>,
    pub end_time: Option<String>,
    pub episode: Option<String>,
    pub episode_id: Option<String>,
    pub episode_number: Option<i32>,
    pub epoch: Option<i64>,
    pub extractor: Option<String>,
    pub extractor_key: Option<String>,
    pub formats: Option<Vec<Format>>,
    pub genre: Option<String>,
    pub id: String,
    pub is_live: Option<bool>,
    pub license: Option<String>,
    pub like_count: Option<i64>,
    pub location: Option<String>,
    pub playlist: Option<String>,
    pub playlist_id: Option<String>,
    pub playlist_index: Option<Value>,
    pub playlist_title: Option<String>,
    pub playlist_uploader: Option<String>,
    pub playlist_uploader_id: Option<String>,
    pub release_date: Option<String>,
    pub release_year: Option<i64>,
    pub repost_count: Option<i64>,
    pub requested_subtitles: Option<BTreeMap<String, Subtitle>>,
    pub season: Option<String>,
    pub season_id: Option<String>,
    pub season_number: Option<i32>,
    pub series: Option<String>,
    pub start_time: Option<String>,
    pub subtitles: Option<BTreeMap<String, Option<Vec<Subtitle>>>>,
    pub tags: Option<Vec<Option<String>>>,
    pub thumbnail: Option<String>,
    pub thumbnails: Option<Vec<Thumbnail>>,
    pub timestamp: Option<f64>,
    pub title: String,
    pub track: Option<String>,
    pub track_id: Option<String>,
    pub track_number: Option<String>,
    pub upload_date: Option<String>,
    pub uploader: Option<String>,
    pub uploader_id: Option<String>,
    pub uploader_url: Option<String>,
    pub view_count: Option<i64>,
    pub webpage_url: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Playlist {
    pub entries: Option<Vec<SingleVideo>>,
    pub extractor: Option<String>,
    pub extractor_key: Option<String>,
    pub id: Option<String>,
    pub title: Option<String>,
    pub uploader: Option<String>,
    pub uploader_id: Option<String>,
    pub uploader_url: Option<String>,
    pub webpage_url: Option<String>,
    pub webpage_url_basename: Option<String>,
    pub thumbnails: Option<Vec<Thumbnail>>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct SingleVideo {
    pub abr: Option<f64>,
    pub acodec: Option<String>,
    pub age_limit: Option<i64>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub album_type: Option<String>,
    pub alt_title: Option<String>,
    pub artist: Option<String>,
    pub asr: Option<f64>,
    pub automatic_captions: Option<BTreeMap<String, Vec<Subtitle>>>,
    pub average_rating: Option<Value>,
    pub categories: Option<Vec<Option<String>>>,
    pub channel: Option<String>,
    pub channel_id: Option<String>,
    pub channel_url: Option<String>,
    pub chapter: Option<String>,
    pub chapter_id: Option<String>,
    pub chapter_number: Option<String>,
    pub chapters: Option<Vec<Chapter>>,
    pub comment_count: Option<i64>,
    pub comments: Option<Vec<Comment>>,
    pub container: Option<String>,
    pub creator: Option<String>,
    pub description: Option<String>,
    pub disc_number: Option<i64>,
    pub dislike_count: Option<i64>,
    pub display_id: Option<String>,
    pub downloader_options: Option<BTreeMap<String, Value>>,
    pub duration: Option<Value>,
    pub duration_string: Option<String>,
    pub end_time: Option<String>,
    pub episode: Option<String>,
    pub episode_id: Option<String>,
    pub episode_number: Option<i32>,
    pub epoch: Option<i64>,
    pub ext: Option<String>,
    pub extractor: Option<String>,
    pub extractor_key: Option<String>,
    pub filesize: Option<i64>,
    pub filesize_approx: Option<f64>,
    pub format: Option<String>,
    pub format_id: Option<String>,
    pub format_note: Option<String>,
    pub formats: Option<Vec<Format>>,
    pub fps: Option<f64>,
    pub fragment_base_url: Option<String>,
    pub fragments: Option<Vec<Fragment>>,
    pub genre: Option<String>,
    pub height: Option<f64>,
    pub http_headers: Option<BTreeMap<String, Option<String>>>,
    pub id: String,
    pub is_live: Option<bool>,
    pub language: Option<String>,
    pub language_preference: Option<i64>,
    pub license: Option<String>,
    pub like_count: Option<i64>,
    pub location: Option<String>,
    pub manifest_url: Option<String>,
    pub no_resume: Option<bool>,
    pub player_url: Option<String>,
    pub playlist: Option<String>,
    pub playlist_id: Option<String>,
    pub playlist_index: Option<Value>,
    pub playlist_title: Option<String>,
    pub playlist_uploader: Option<String>,
    pub playlist_uploader_id: Option<String>,
    pub preference: Option<Value>,
    pub protocol: Option<Protocol>,
    pub quality: Option<f64>,
    pub release_date: Option<String>,
    pub release_year: Option<i64>,
    pub repost_count: Option<i64>,
    pub requested_subtitles: Option<BTreeMap<String, Subtitle>>,
    pub resolution: Option<String>,
    pub season: Option<String>,
    pub season_id: Option<String>,
    pub season_number: Option<i32>,
    pub series: Option<String>,
    pub source_preference: Option<i64>,
    pub start_time: Option<String>,
    pub stretched_ratio: Option<f64>,
    pub subtitles: Option<BTreeMap<String, Option<Vec<Subtitle>>>>,
    pub tags: Option<Vec<Option<String>>>,
    pub tbr: Option<f64>,
    pub thumbnail: Option<String>,
    pub thumbnails: Option<Vec<Thumbnail>>,
    pub timestamp: Option<f64>,
    pub title: String,
    pub track: Option<String>,
    pub track_id: Option<String>,
    pub track_number: Option<String>,
    pub upload_date: Option<String>,
    pub uploader: Option<String>,
    pub uploader_id: Option<String>,
    pub uploader_url: Option<String>,
    pub url: Option<String>,
    pub vbr: Option<f64>,
    pub vcodec: Option<String>,
    pub view_count: Option<i64>,
    pub webpage_url: Option<String>,
    pub width: Option<f64>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Subtitle {
    pub data: Option<String>,
    pub ext: Option<String>,
    pub url: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Thumbnail {
    pub filesize: Option<i64>,
    pub height: Option<f64>,
    pub id: Option<String>,
    pub preference: Option<i64>,
    pub url: Option<String>,
    pub width: Option<f64>,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum Protocol {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "https")]
    Https,
    #[serde(rename = "rtsp")]
    Rtsp,
    #[serde(rename = "rtmp")]
    Rtmp,
    #[serde(rename = "rtmpe")]
    Rtmpe,
    #[serde(rename = "mms")]
    Mms,
    #[serde(rename = "f4m")]
    F4M,
    #[serde(rename = "ism")]
    Ism,
    #[serde(rename = "m3u8")]
    M3U8,
    #[serde(rename = "m3u8_native")]
    M3U8Native,
    #[serde(rename = "http_dash_segments")]
    HttpDashSegments,
    #[serde(rename = "mhtml")]
    Mhtml,
    #[serde(rename = "https+https")]
    HttpsHttps,
    #[serde(rename = "http_dash_segments+https")]
    HttpDashSegmentsHttps,
    #[serde(rename = "http_dash_segments+http_dash_segments")]
    HttpDashSegmentsHttpDashSegments,
    #[serde(rename = "m3u8_native+m3u8_native")]
    M3U8NativeM3U8Native,
}

// Codec values are set explicitly, and when there is no codec, it is sometimes
// given as "none" (instead of simply missing from the JSON).
// Default decoding in this case would result in `Some("none".to_string())`, which is why
// this custom parse function exists.
fn parse_codec<'de, D>(d: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    Deserialize::deserialize(d).map(|x: Option<_>| match x.unwrap_or_default() {
        Some(s) if s == "none" => None,
        x => x,
    })
}
