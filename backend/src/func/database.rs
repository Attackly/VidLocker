use sqlx::postgres::PgPoolOptions;

pub async fn write_video_infromation(pool: PgPool, viewkey: String) {
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

    sqlx::q
}
