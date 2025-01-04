use std::{collections::HashMap, env};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize,Serialize};
use reqwest;

#[derive(Debug)]
pub struct Video {
    viewkey: String,
    published_at: NaiveDateTime,
    channel_id: String,
    title: String,
    description: String,
    channel_name: String,
    tags: Vec<String>,
}

#[derive(Serialize,Deserialize, Debug)]
struct YouTubeApiResponse {
    items: Vec<YouTubeVideoItem>,
}

#[derive(Deserialize, Serialize, Debug)]
struct YouTubeVideoItem {
    snippet: VideoSnippet,
}

#[derive(Deserialize, Serialize, Debug)]
struct VideoSnippet {
    title: String,
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

    let res = reqwest::get(&url).await.unwrap()
        .json::<YouTubeApiResponse>().await.unwrap();
    
    println!("{:?}", res);  

    if let Some(first_item) = res.items.get(0) {
        println!("Video Title: {}", first_item.snippet.title);
    } else {
        println!("No video found for the given ID");
    }

    Some(Video {
        viewkey: viewkey.to_owned(),
        published_at: Utc::now().naive_utc(),
        channel_id: "".to_owned(),
        title: "".to_owned(),
        description: "".to_owned(),
        channel_name: "".to_owned(),
        tags: vec![]
    })
    
}


#[tokio::test]
async fn test_get_title() {
    dotenvy::dotenv().ok();
    let res = get_title("dQw4w9WgXcQ").await.unwrap();
    println!("{:?}", res)
}