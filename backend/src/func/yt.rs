use chrono::{DateTime, Utc};
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Value};
use std::env;
use std::process::{Command, Stdio};
use tokio::time::Instant;
use tracing::{debug, error, info};

pub fn get_mode() -> String {
    if env::var("YT_API_KEY").is_err() {
        "fallback".to_owned()
    } else {
        "api".to_owned()
    }
}

pub async fn get_title(viewkey: &str) -> Option<VideoResp> {
    match env::var("YT_API_KEY") {
        Ok(key) => return get_title_api(viewkey, key).await,
        Err(_) => (),
    };

    let start = Instant::now();
    let output = Command::new("yt-dlp").arg("-j").arg(&viewkey).stdout(Stdio::piped()).stderr(Stdio::piped()).output().unwrap();
    if !output.status.success() {
        error!("The Error returend form yt-dlp: {:?}", output.stderr);
        return None
    }

    let json_output = String::from_utf8(output.stdout).unwrap();

    let yt_dlp_details: VideoDetails = from_str(&json_output).unwrap();

    let upload_date = yt_dlp_details.timestamp
        .and_then(|ts| DateTime::from_timestamp(ts, 0));

    let final_details = VideoResp {
        viewkey: viewkey.to_string(),
        published_at: upload_date,
        channel_id: None,
        title: Some(yt_dlp_details.title),
        description: Some(yt_dlp_details.description),
        channel_name: Some(yt_dlp_details.channel),
        tags: Some(yt_dlp_details.tags),
    };
    let duration = start.elapsed();
    debug!("The VideoResp {:?}", final_details);
    debug!("yt-dlp needed this time to fetch: {:?}", duration);
    error!("Test 2");
    Some(final_details)
}

async fn get_title_api(viewkey: &str, key: String) -> Option<VideoResp> {
    let url = format!(
        "https://www.googleapis.com/youtube/v3/videos?id={}&part=snippet&key={}",
        viewkey, key
    );

    let response = reqwest::get(&url).await.unwrap();

    let json: Value = response.json().await.unwrap();
    println!("{:?}", json);

    if let Some(items) = json.get("items") {
        if let Some(first_item) = items.get(0) {
            if let Some(snippet) = first_item.get("snippet") {
                if let Some(title) = snippet.get("title") {
                    println!("Video Title: {}", title);
                }
            }
        }
    }


    info!("Datum was sollte: {:?}", json["items"][0]["snippet"]["publishedAt"]);
    let parsed_date = Some(
        DateTime::parse_from_rfc3339(json["items"][0]["snippet"]["publishedAt"].as_str().unwrap())
            .unwrap()
            .with_timezone(&Utc),
    );

    let tags: Vec<String> = json["items"][0]["snippet"]["tags"]
        .as_array()
        .unwrap_or(&vec![]) // Use an empty vector if tags are missing or null
        .iter()
        .map(|tag| tag.as_str().unwrap_or("").to_string()) // Handle potential non-string tags
        .collect();

    let title = json["items"][0]["snippet"]["title"]
        .as_str()
        .map(|t| t.replace(r#"\""#, r#"""#))
        .unwrap_or_else(|| "Default Title".to_string());

    let channel_name = json["items"][0]["snippet"]["channelTitle"]
        .as_str()
        .map(|t| t.replace(r#"\""#, r#"""#))
        .unwrap_or_else(|| "Default Title".to_string());

    Some(VideoResp {
        viewkey: viewkey.to_owned(),
        published_at: parsed_date,
        channel_id: Some(json["items"][0]["snippet"]["channelId"].to_string()),
        title: Some(title),
        description: Some(json["items"][0]["snippet"]["description"].to_string()),
        channel_name: Some(channel_name),
        tags: Some(tags),
    })
}

#[derive(Serialize)]
#[derive(Debug)]
pub struct VideoResp {
    viewkey: String,
    published_at: Option<DateTime<Utc>>,
    pub channel_id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub channel_name: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct VideoDetails {
    title: String,
    description: String,
    channel: String,
    tags: Vec<String>,
    timestamp: Option<i64>
}