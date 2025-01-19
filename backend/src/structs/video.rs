use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct Video {
    viewkey: String,
    published_at: Option<DateTime<Utc>>,
    channel_id: Option<String>,
    title: Option<String>,
    description: Option<String>,
    channel_name: Option<String>,
    tags: Option<Vec<String>>,
    url: Url,
    created_at: Option<DateTime<Utc>>,
    downloaded_at: Option<DateTime<Utc>>,
    path: Option<PathBuf>,
    duration: Option<u32>,
    viewcount: Option<u64>,
    ext: Option<String>,
    lang: Option<String>,
    height: Option<u16>,
    width: Option<u16>,
    dynamic_range: Option<String>,
    availability: Option<String>,
    fps: Option<u16>,
    average_rating: Option<u8>,
    age_limit: Option<u8>,
    likes: Option<u64>,
    status: Option<String>,
    comments: Option<u64>,
    chapters: Option<String>,
}

impl Video {
    fn from_yt_viewkey(viwkey: String) -> Video {
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

        let published_at = match json.get("timestamp") {
            Some(v) => v.to_string().parse::<i64>().unwrap(),
            None(_) => 0,
        };

        let channel_id = json
            .get("channel_id")
            .and_then(|u| u.as_str())
            .map(String::from)
            .unwrap_or_else(|| "Unknown Uploader".to_string());

        let uploader = json
            .get("uploader")
            .and_then(|u| u.as_str())
            .map(String::from) // Convert to String
            .unwrap_or_else(|| "Unknown uploader".to_string());

        let title = json
            .get("title")
            .and_then(|t| t.as_str())
            .map(String::from)
            .unwrap_or_else(|| "Unkown Title");

        let description = json
            .get("description")
            .and_then(|d| d.as_str())
            .map(String::from)
            .unwrap_or_else(|| "No Description");

        let channel_name = json
            .get("channel")
            .and_then(|d| d.as_str())
            .map(String::from)
            .unwrap_or_else(|| "No Description");

        let tags = json.get("tags");

        let url = json.get("url");

        let created_at = DateTime::now();

        let duration = json.get("duration").and_then(|d| d.parse::<u32>());

        let viewcount = json.get("view_count").and_then(|d| d.parse::<u64>());

        let lang = json
            .get("language")
            .and_then(|l| l.as_str())
            .map(String::from)
            .unwrap_or_else(|| "No Description");

        let height = json.get("height").and_then(|h| h.parse::<u16>());
        let width = json.get("width").and_then(|w| w.parse::<u16>());

        let dynamic_range = json
            .get("dynamic_range")
            .and_then(|l| l.as_str())
            .map(String::from)
            .unwrap_or_else(|| "N/a");

        let availability = json
            .get("availability")
            .and_then(|a| a.as_str())
            .map(String::from)
            .unwrap_or_else(|| "N/a");

        let fps = json.get("fps").and_then(|f| f.parse::<u16>());

        let average_rating = json
            .get("average_rating")
            .and_then(|a| a.as_str())
            .map(String::from)
            .unwrap_or_else(|| None);

        let age_limit = json.get("age_limit").and_then(|a| a.parse::<u8>());

        let likes = json.get("like_count").and_then(|a| a.parse::<u64>());

        let status = json
            .get("status")
            .and_then(|a| a.as_str())
            .map(String::from)
            .unwrap_or_else(|| "N/a");

        let comments = json
            .get("comment_count")
            .and_then(|a| a.parse::<u64>().unwrap_or_else(|| 0));

        let chapters = json
            .get("chapters")
            .and_then(|a| a.as_str())
            .map(String::from)
            .unwrap_or_else(|| "N/a");

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
            path,
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
        }
    }
}
