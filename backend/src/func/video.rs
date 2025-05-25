use sqlx::postgres::PgPoolOptions;
use std::path::PathBuf;
use std::thread;
use tracing::error;
use youtube_dl::YoutubeDl;

use crate::structs::video::Video;

pub async fn download_video_simple_ydl(link: String, path: String) {
    // Download the video in a separate thread
    thread::spawn(
        move || match YoutubeDl::new(link).download_to(PathBuf::from(&path)) {
            Ok(_) => println!("Video downloaded successfully to '{}'", path),
            Err(e) => eprintln!("Failed to download video: {}", e),
        },
    );
    return;
}

pub async fn write_db_entry(link: &String) {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let url: Vec<&str> = { link.split("?v=").collect() };

    let res = Video::from_yt_viewkey(url[1].to_string());

    let id = res.to_database(&pool).await;

    let queue_res = sqlx::query!(
        "INSERT INTO queue (video_id, added_by) VALUES ($1, 1)",
        id.unwrap()
    )
    .execute(&pool)
    .await;

    match queue_res {
        Ok(_) => {}
        Err(e) => {
            error!("There was an error writing the Video into the Queue: {e}")
        }
    }
}
