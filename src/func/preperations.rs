use std::{fs, io};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;   

pub fn create_output_dir() -> (){
    match fs::create_dir("output") {
        Ok(_) => (),
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
            ();
        }
        Err(e) => panic!("There was an error! {e}")
    }
}

pub async fn prepare_database() -> (){
    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new().max_connections(30).connect(&database_url).await.expect("Failed to connect to database");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
}