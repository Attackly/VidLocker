use sqlx::postgres::PgPoolOptions;

pub async fn write_video_information(pool: PgPool, viewkey: String) {
    sqlx::query!("INSERT INTO");
}
