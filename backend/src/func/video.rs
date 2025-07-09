use std::env::current_dir;
use std::path::PathBuf;
use std::thread;
use tracing::{error, info};
use youtube_dl::YoutubeDl;
use crate::AppState;
use crate::structs::video::Video;

pub async fn download_video_simple_ydl(link: String, path: String) {
    // Download the video in a separate thread
    thread::spawn(
        move || match YoutubeDl::new(link).download_to(PathBuf::from(&format!("./output/{}", path))) {
            Ok(_) => info!("Video downloaded successfully to '{}', current path: {:?}", path, current_dir()),
            Err(e) => eprintln!("Failed to download video: {}", e),
        },

    );
    return;
}

pub async fn write_db_entry(link: &String,  state: AppState) {

    let url: Vec<&str> = { link.split("?v=").collect() };

    let res = Video::from_yt_viewkey(url[1].to_string());

    let mut conn = state.db_pool.acquire().await.unwrap();
    let id = res.to_database(&mut conn).await;

    let queue_res = sqlx::query!(
        "INSERT INTO queue (video_id, added_by) VALUES ($1, 1)",
        id.unwrap()
    )
    .execute(&mut *conn)
    .await;

    match queue_res {
        Ok(_) => {}
        Err(e) => {
            error!("There was an error writing the Video into the Queue: {e}")
        }
    }
}
