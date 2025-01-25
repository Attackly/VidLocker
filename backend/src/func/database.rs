use sqlx::postgres::PgPoolOptions;

pub async fn write_video_infromation(pool: PgPool, viewkey: String) {
    sqlx::query!("INSERT INTO");
}
