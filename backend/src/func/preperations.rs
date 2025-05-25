use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::{fs, io};

pub fn create_output_dir() -> () {
    match fs::create_dir_all("output/shared") {
        Ok(_) => (),
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
            ();
        }
        Err(e) => panic!("There was an error! {e}"),
    }
}

pub async fn prepare_database() -> () {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(30)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
}

#[cfg(test)]
mod test {
    /*
    COMMENTED OUT BECAUSE SUPABASE CANT CREATE NEW DATABASES ON THE FLY

    use sqlx::PgPool;

    #[sqlx::test]
    async fn test_prepare_database(pool: PgPool) -> sqlx::Result<()> {
        let mut conn = pool.acquire().await?;

        sqlx::migrate!("./migrations").run(&mut conn).await.unwrap();
        Ok(())
    }
    */
}
