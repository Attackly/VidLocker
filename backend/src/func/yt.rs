use chrono::{DateTime, Utc};
use reqwest;
use serde::Serialize;
use serde_json::Value;
use std::{env, process::Command};

#[derive(Debug, Serialize)]
pub struct Video {
    viewkey: String,
    published_at: DateTime<Utc>,
    channel_id: String,
    title: String,
    description: String,
    channel_name: String,
    tags: Vec<String>,
}

pub fn get_mode() -> String {
    if env::var("YT_API_KEY").is_err() {
        "fallback".to_owned()
    } else {
        "api".to_owned()
    }
}

pub async fn get_title(viewkey: &str) -> Option<Video> {
    match env::var("YT_API_KEY") {
        Ok(key) => return get_title_api(viewkey, key).await,
        Err(_) => return get_title_fallback(viewkey).await,
    };
}

async fn get_title_fallback(viewkey: &str) -> Option<Video> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("yt-dlp -J {}", viewkey))
        .output()
        .expect("Failed to execute command");

    let json: Value = serde_json::from_str(
        String::from_utf8(output.stdout)
            .expect("yt-dlp did not give a Valid Json output")
            .as_str(),
    )
    .unwrap();

    let timestamp = json
        .get("timestamp")
        .unwrap()
        .to_string()
        .parse::<i64>()
        .unwrap();

    // Create a normal DateTime from the NaiveDateTime
    let datetime: DateTime<Utc> = DateTime::from_timestamp(timestamp, 0).unwrap();

    let title = json
        .get("title")
        .and_then(|t| t.as_str())
        .map(String::from) // Convert to String
        .unwrap_or_else(|| "Unknown title".to_string());

    // Extract and convert uploader to String
    let uploader = json
        .get("uploader")
        .and_then(|u| u.as_str())
        .map(String::from) // Convert to String
        .unwrap_or_else(|| "Unknown uploader".to_string());

    Some(Video {
        viewkey: viewkey.to_owned(),
        published_at: datetime,
        channel_id: "".to_owned(),
        title,
        description: "".to_owned(),
        channel_name: uploader,
        tags: vec![],
    })
}

async fn get_title_api(viewkey: &str, key: String) -> Option<Video> {
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
    println!("{:?}", json["items"][0]["snippet"]["title"]);
    println!("{:?}", json["items"][0]["snippet"]["channelId"]);

    println!("{:?}", json["items"][0]["snippet"]["publishedAt"]);
    let parsed_date =
        DateTime::parse_from_rfc3339(json["items"][0]["snippet"]["publishedAt"].as_str().unwrap())
            .unwrap()
            .with_timezone(&Utc);

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

    Some(Video {
        viewkey: viewkey.to_owned(),
        published_at: parsed_date,
        channel_id: json["items"][0]["snippet"]["channelId"].to_string(),
        title: title,
        description: json["items"][0]["snippet"]["description"].to_string(),
        channel_name: channel_name,
        tags: tags,
    })
}

/*
#[tokio::test]
async fn test_get_title() {
    dotenvy::dotenv().ok();
    let res = get_title("dQw4w9WgXcQ").await.unwrap();
    println!("{:?}", res)
}
*/

#[tokio::test]
async fn get_title_test() {
    let res = get_title_fallback("dQw4w9WgXcQ").await.unwrap();
    println!("{:?}", res)
}
