use serde_json::Value;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use youtube_dl::YoutubeDl; // Assuming you're using the youtube-dl crate.

pub async fn download_video_simple_ydl(link: String) {
    write_db_entry(&link).await;

    if env::var("MODE").unwrap() != "queue" {
        let folder_path = "/home/jan/Projects/VidLocker/output";

        // Download the video in a separate threadis_ok
        thread::spawn(
            move || match YoutubeDl::new(link).download_to(PathBuf::from(folder_path)) {
                Ok(_) => println!("Video downloaded successfully to '{}'", folder_path),
                Err(e) => eprintln!("Failed to download video: {}", e),
            },
        );

        return;
    }

    // sqlx::query!("INSERT INTO queue (video_id) VALUES ()")
}

async fn write_db_entry(link: &String) {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("yt-dlp -J {}", link))
        .output()
        .expect("Failed to execute command");

    let json_output = String::from_utf8(output.stdout).expect("Invalid UTF-8 in command output");
    let json_value: Value = serde_json::from_str(&json_output).expect("Failed to parse JSON");

    println!("{:?}", json_value.get("id").unwrap().to_string());
    let res =
        sqlx::query!(
        "INSERT INTO videos (viewkey, title, description, url, path) VALUES ($1, $2, $3, $4, $5) RETURNING id;",
        json_value.get("id").unwrap().to_string().replace("\"", ""),
        json_value.get("title").unwrap().to_string().replace("\"",""),
        json_value.get("description").unwrap().to_string().replace("\"",""),
        link,
        "/"
    )
        .fetch_one(&pool)
        .await
        .unwrap();

    sqlx::query!("INSERT INTO queue (video_id) VALUES ($1)", res.id)
        .execute(&pool)
        .await
        .unwrap();
    // Write the entry to the database
}

#[tokio::test]
async fn test_download_video_simple_ydl() {
    dotenvy::dotenv().ok();
    let res = write_db_entry(&"https://www.youtube.com/watch?v=Rv4zAZMiS4Y".to_string()).await;
    println!("{:?}", res);

    return;
}
