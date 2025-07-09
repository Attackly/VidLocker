use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::{env, fs, io};
use std::io::ErrorKind;
use tracing::{info, warn};
use crate::func::files::dir_create;

pub async fn create_output_dir() -> () {
    match fs::create_dir_all("output/shared") {
        Ok(_) => (),
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
            ();
        }
        Err(e) => panic!("There was an error! {e}"),
    }

    let auth = env::var("AUTH");
    if auth.is_ok() && auth.unwrap().to_lowercase() == "true" {
        let conn = PgPoolOptions::new().connect(env::var("DATABASE_URL").unwrap().as_ref()).await.unwrap();
        let query = sqlx::query!("SELECT * FROM users").fetch_all(&conn).await;
        if let Ok(query) = query {
            for user in query {
                match dir_create("output/".to_owned() + &user.username.as_str()).await {
                    Ok(_) => {info!("Created output dir: {}", user.username.as_str());}
                    Err(e) => match e.kind() {
                        ErrorKind::AlreadyExists => {
                            warn!("Directory already exists: {}", user.username.as_str());
                        }
                        ErrorKind::InvalidFilename => {
                            warn!("User {} has an Name that would result in an Invalid dir name. Skipping this user", user.username.as_str());
                        }
                        e => panic!("Cant create output dir due to unkown error: This message may give more information: {}", e),
                    }
                };
            }
        }
    }
    else {
        warn!("AUTH env var is not set or not set to true; Continuing without User dirs")
    }
}

pub async fn prepare_database() -> () {
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(2)
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
