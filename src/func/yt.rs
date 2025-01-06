use std::env;
use chrono::{DateTime, Utc};
use reqwest;
use serde::Serialize;
use serde_json::Value;




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
    }
    else {
        "api".to_owned()
    }
}


pub async fn get_title(viewkey: &str) -> Option<Video> {
    
    let key = match env::var("YT_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            return None;
        }
    };

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
    let parsed_date = DateTime::parse_from_rfc3339(json["items"][0]["snippet"]["publishedAt"].as_str().unwrap()).unwrap().with_timezone(&Utc);

    let tags: Vec<String> = json["items"][0]["snippet"]["tags"]
        .as_array()
        .unwrap_or(&vec![]) // Use an empty vector if tags are missing or null
        .iter()
        .map(|tag| tag.as_str().unwrap_or("").to_string()) // Handle potential non-string tags
        .collect();

        let title = json["items"][0]["snippet"]["title"]
        .as_str()
        .map(|t| {
            t.replace(r#"\""#, r#"""#)
        })
        .unwrap_or_else(|| "Default Title".to_string());

        let channel_name = json["items"][0]["snippet"]["channelTitle"]
        .as_str()
        .map(|t| {
            t.replace(r#"\""#, r#"""#)
        })
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