use regex::Regex;
use reqwest::Client;
use serde::Deserialize;
use std::error::Error;
use std::time::Duration;
use tracing::error;
use crate::structs::video::Video;
use chrono::{DateTime, Utc};
use reqwest;
use serde::Serialize;
use serde_json::Value;
use std::env;

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

    if viewkey.len() == 11 {
        return Some(get_youtube_details_embedded_json(&*("https://youtube.com/watch?v=".to_owned() + viewkey)).await.unwrap());
    }

    return Some(get_youtube_details_embedded_json(viewkey).await.unwrap());
}

async fn get_title_api(viewkey: &str, key: String) -> Option<VideoResp> {
    let url = format!(
        "https://www.googleapis.com/youtube/v3/videos?id={}&part=snippet&key={}",
        viewkey, key
    );

    let response = reqwest::get(&url).await.unwrap();

    // Parse the response as a dynamic JSON
    let json: Value = response.json().await.unwrap();
    println!("{:?}", json);

    // Access specific fields dynamically (example: video title)
    if let Some(items) = json.get("items") {
        if let Some(first_item) = items.get(0) {
            if let Some(snippet) = first_item.get("snippet") {
                if let Some(title) = snippet.get("title") {
                    println!("Video Title: {}", title);
                }
            }
        }
    }

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

    // Think what to do off this.
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

// TODO FIX THIS. THATS A STUPID AND BAD SOLUTION TO THE PROBLEM
#[derive(Serialize)]
pub struct VideoResp {
    viewkey: String,
    published_at: Option<DateTime<Utc>>,
    pub channel_id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub channel_name: Option<String>,
    pub tags: Option<Vec<String>>,
}


#[derive(Deserialize, Debug)]
struct PlayerMicroformatRenderer {
    #[serde(rename = "title")]
    title_object: Option<TitleObject>,
    #[serde(rename = "ownerChannelName")]
    owner_channel_name: Option<String>,
    #[serde(rename = "uploadDate")]
    upload_date: Option<String>, // Typically YYYY-MM-DD
    #[serde(rename = "publishDate")]
    publish_date: Option<String>, // Also YYYY-MM-DD, fallback
}

#[derive(Deserialize, Debug)]
struct TitleObject {
    #[serde(rename = "simpleText")]
    simple_text: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Microformat {
    #[serde(rename = "playerMicroformatRenderer")]
    player_microformat_renderer: Option<PlayerMicroformatRenderer>,
}

#[derive(Deserialize, Debug)]
struct YtInitialPlayerResponse {
    microformat: Option<Microformat>,
}

pub async fn get_youtube_details_embedded_json(
    video_url: &str,
) -> Result<VideoResp, Box<dyn Error>> {
    let mut details = VideoResp {
        viewkey: "".to_string(),
        published_at: None,
        channel_id: None,
        title: None,
        description: None,
        channel_name: None,
        tags: None,
    };

    let client = Client::builder().timeout(Duration::from_secs(10)).build()?;

    let response = client.get(video_url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .header("Accept-Language", "en-US,en;q=0.9")
        .send().await?;

    if !response.status().is_success() {
        return Err(format!("Request failed with status: {}", response.status()).into());
    }

    let html_content = response.text().await?;
    let re = Regex::new(r"ytInitialPlayerResponse\s*=\s*(\{.+?});")?;

    if let Some(caps) = re.captures(&html_content) {
        if let Some(match_group) = caps.get(1) {
            let json_str = match_group.as_str();
            if let Ok(parsed_response) = serde_json::from_str::<YtInitialPlayerResponse>(json_str) {
                if let Some(microformat) = parsed_response.microformat {
                    if let Some(renderer) = microformat.player_microformat_renderer {
                        if let Some(title_obj) = renderer.title_object {
                            details.title = title_obj.simple_text;
                        }
                        details.channel_name = renderer.owner_channel_name;
                        details.published_at = Some(
                            renderer
                                .upload_date
                                .or(renderer.publish_date)
                                .unwrap()
                                .parse()
                                .unwrap(),
                        );

                        if details.title.is_some()
                            && details.channel_name.is_some()
                            && details.published_at.is_some()
                        {
                            return Ok(details);
                        }
                    }
                }
            } else {
                error!("error parsing yt repsonse");
            }
        }
    } else {
        error!("error parsing yt repsonse");
    }

    if details.title.is_none() && details.published_at.is_none() && details.published_at.is_none() {
        Err("Could not extract any details from embedded JSON.".into())
    } else {
        Ok(details)
    }
}
