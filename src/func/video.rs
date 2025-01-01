use std::path::PathBuf;
use std::thread;
use youtube_dl::YoutubeDl; // Assuming you're using the youtube-dl crate.

pub fn download_video_simple_ydl(link: String) {
    let folder_path = "/home/jan/Projects/VidLocker/output";
    
    // Download the video in a separate thread
    thread::spawn(move || {
        match YoutubeDl::new(link).download_to(PathBuf::from(folder_path)) {
            Ok(_) => println!("Video downloaded successfully to '{}'", folder_path),
            Err(e) => eprintln!("Failed to download video: {}", e),
        }
    });
}