use serde::Serialize;
use serde::Serializer;
use serde_json::Value;
use serde_with::serde_as;
use sqlx::Postgres;
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::time::OffsetDateTime;
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use sqlx::pool::PoolConnection;
use tracing::info;
use tracing::warn;
use url::Url;

#[serde_as]
#[derive(Debug, Serialize)]
pub struct Video {
    pub viewkey: String,
    pub published_at: Option<DateTime<Utc>>,
    pub channel_id: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub channel_name: Option<String>,
    pub tags: Option<Vec<String>>,
    #[serde(serialize_with = "serialize_option_url")]
    pub url: Option<Url>,
    pub created_at: Option<DateTime<Utc>>,
    pub downloaded_at: Option<DateTime<Utc>>,
    pub path: Option<PathBuf>,
    pub duration: Option<u32>,
    pub viewcount: Option<u64>,
    pub ext: Option<String>,
    pub lang: Option<String>,
    pub height: Option<u16>,
    pub width: Option<u16>,
    pub dynamic_range: Option<String>,
    pub availability: Option<String>,
    pub fps: Option<u16>,
    pub average_rating: Option<u8>,
    pub age_limit: Option<u8>,
    pub likes: Option<u64>,
    pub status: Option<String>,
    pub comments: Option<u64>,
    pub chapters: Option<String>,
}

impl Video {
    pub fn from_yt_viewkey(viewkey: String) -> Video {
        info!("The viewkey: {}", viewkey);
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

        let timestamp;
        let published_at;
        if json.get("timestamp").is_none() {
            warn!("Timestamp contains None");
            published_at = None;
        } else {
            timestamp = json.get("timestamp").and_then(|f| f.as_i64());
            published_at =
                Some(DateTime::from_timestamp(timestamp.unwrap(), 0).expect("Invalid timestamp"));
        }

        let channel_id = match json
            .get("channel_id")
            .and_then(|u| u.as_str())
            .map(String::from)
        {
            Some(v) => Some(v),
            None => None,
        };

        let title = match json.get("title").and_then(|t| t.as_str()).map(String::from) {
            Some(v) => Some(v),
            None => None,
        };

        let description = match json
            .get("description")
            .and_then(|d| d.as_str())
            .map(String::from)
        {
            Some(v) => Some(v),
            None => None,
        };

        let channel_name = match json
            .get("channel")
            .and_then(|d| d.as_str())
            .map(String::from)
        {
            Some(v) => Some(v),
            None => None,
        };

        let tags: Option<Vec<String>> = json
            .get("tags")
            .and_then(|tags| tags.as_array()) // Ensure it's an array
            .map(|array| {
                array
                    .iter()
                    .filter_map(|value| value.as_str().map(|s| s.to_string())) // Convert to String
                    .collect()
            });

        let url = match Url::from_str(json.get("webpage_url").unwrap().as_str().unwrap()) {
            Ok(v) => Some(v),
            Err(_) => None,
        };

        let created_at = Some(Utc::now());

        let duration = json
            .get("duration")
            .and_then(|d| d.as_u64())
            .map(|d| d as u32);

        let viewcount = json.get("view_count").and_then(|a| a.as_u64());

        let lang = match json
            .get("language")
            .and_then(|l| l.as_str())
            .map(String::from)
        {
            Some(v) => Some(v),
            None => None,
        };

        let height = json
            .get("height")
            .and_then(|a| a.as_u64())
            .map(|a| a as u16);

        let width = json.get("width").and_then(|a| a.as_u64()).map(|a| a as u16);

        let dynamic_range = match json
            .get("dynamic_range")
            .and_then(|l| l.as_str())
            .map(String::from)
        {
            Some(v) => Some(v),
            None => None,
        };

        let availability = match json
            .get("availability")
            .and_then(|a| a.as_str())
            .map(String::from)
        {
            Some(v) => Some(v),
            None => None,
        };

        let fps = json.get("fps").and_then(|a| a.as_u64()).map(|a| a as u16);

        let average_rating: Option<u8> = json
            .get("average_rating")
            .and_then(|a| a.as_u64())
            .map(|val| val as u8);

        let age_limit = json
            .get("age_limit")
            .and_then(|a| a.as_u64())
            .map(|a| a as u8);

        let likes = json.get("like_count").and_then(|a| a.as_u64());

        let status = match json
            .get("status")
            .and_then(|a| a.as_str())
            .map(String::from)
        {
            Some(v) => Some(v),
            None => None,
        };

        let comments = json.get("comment_count").and_then(|a| a.as_u64());

        let chapters = match json
            .get("chapters")
            .and_then(|a| a.as_str())
            .map(String::from)
        {
            Some(v) => Some(v),
            None => None,
        };

        Video {
            viewkey,
            published_at,
            channel_id,
            title,
            description,
            channel_name,
            tags,
            url,
            created_at,
            duration,
            viewcount,
            lang,
            height,
            width,
            dynamic_range,
            availability,
            fps,
            average_rating,
            age_limit,
            likes,
            status,
            comments,
            chapters,
            downloaded_at: None,
            ext: None,
            path: Some(PathBuf::from("/")),
        }
    }
    pub async fn to_database(self, pool: &mut PoolConnection<Postgres>) -> Result<i32, sqlx::Error> {
        match sqlx::query!("INSERT INTO videos (viewkey, title, description, url, created_at, downloaded_at, duration, viewcount, ext, lang, height, width, dynamic_range, availability, fps, average_rating, age_limit, likes, status, comments, chapters, path) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, '/') RETURNING id", self.viewkey, self.title, self.description, self.url.map(|url| url.to_string()),   self.created_at.map(|dt| OffsetDateTime::from_unix_timestamp(dt.timestamp()).unwrap().replace_nanosecond(dt.timestamp_subsec_nanos()).unwrap()),
          self.downloaded_at.map(|dt| OffsetDateTime::from_unix_timestamp(dt.timestamp()).unwrap().replace_nanosecond(dt.timestamp_subsec_nanos()).unwrap())
            , self.duration.map(|duration| duration as i32), self.viewcount.map(|vc| vc as i64), self.ext, self.lang, self.height.map(|height| height as i32), self.width.map(|width| width as i32), self.dynamic_range, self.availability, self.fps.map(|fps| fps as i32), self.average_rating.map(|ar| ar as i16), self.age_limit.map(|al| al as i16), self.likes.map(|likes| likes as i32), self.status, self.comments.map(|comments| comments as i32), self.chapters)
            .fetch_one(&mut **pool).await {
            Ok(id) => Ok(id.id),
            Err(e) => {println!("{:?}", e); Err(e)}
        }
    }
}

fn serialize_option_url<S>(url: &Option<Url>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match url {
        Some(u) => serializer.serialize_some(u.as_str()), // Serialize Url as a string
        None => serializer.serialize_none(),              // Serialize None
    }
}
