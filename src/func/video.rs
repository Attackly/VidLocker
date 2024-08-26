use std::path::PathBuf;
use std::thread;
use youtube_dl::{YoutubeDl};
pub fn download_video_simple_ydl(link: String) {
    thread::spawn(move || {
        YoutubeDl::new(link).download_to(PathBuf::from("output/")).unwrap();
    });
}