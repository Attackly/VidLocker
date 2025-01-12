#[deny(warnings)]
mod func;
mod queue;
mod routes;

use std::time::Duration;

use crate::func::preperations::create_output_dir;
use crate::queue::queue_worker::queue_worker;
use crate::routes::files::{create_dir_handler, get_single_dir_size_handler};
use crate::routes::video::simple_download_handler;
use crate::routes::yt::title_handler;
use axum::http::Method;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use func::preperations::prepare_database;
use routes::{files::dir_delete_handler, misc::check_system_handler, yt::mode_handler};
use sqlx::postgres::PgPoolOptions;
use tokio::time::sleep;
use tower_http::cors::{Any, CorsLayer};
#[tokio::main]
async fn main() {
    let handle_count = 4;
    let idle_time = 10;

    create_output_dir();
    prepare_database().await;

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let queue_pool = PgPoolOptions::new()
        .max_connections(handle_count)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let mut handles = Vec::new();
    for i in 0..handle_count {
        let pool = queue_pool.clone();
        let handle = tokio::spawn(async move {
            println!("Started worker {i}");
            queue_worker(i, pool).await;
        });
        handles.push(handle);
        sleep(Duration::from_secs((idle_time / handle_count) as u64)).await;
    }

    // create_download_threads();
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_origin(Any) // Allow any origin
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]) // Specify allowed methods
        .allow_headers([axum::http::header::CONTENT_TYPE]); // Allow specific headers

    let app = Router::new()
        .route("/test", get(check_system_handler))
        .route("/api/downloadVideo", post(simple_download_handler))
        .route("/api/files/size", post(get_single_dir_size_handler))
        .route("/api/files/dir_delete", delete(dir_delete_handler))
        .route("/api/yt/mode", get(mode_handler))
        .layer(cors.clone())
        .route("/api/yt/getTitle", post(title_handler))
        .layer(cors.clone())
        .route("/api/files/dir_create", put(create_dir_handler))
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap()
}
