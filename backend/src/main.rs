// #[deny(warnings)]
mod func;
mod queue;
mod routes;
mod structs;
use crate::{
    func::preperations::{create_output_dir, prepare_database},
    queue::queue_worker::queue_worker,
    routes::{
        files::{create_dir_handler, dir_delete_handler, get_single_dir_size_handler, list_files},
        misc::check_system_handler,
        video::simple_download_handler,
        yt::{mode_handler, title_handler},
    },
};

use axum::http::{Request, StatusCode};
use axum::response::IntoResponse;
use axum::response::Response;
use std::{convert::Infallible, net::SocketAddr, path::PathBuf};
use tokio::fs;
use tower::service_fn;
use tower_http::services::ServeDir;

use axum::{
    Router,
    body::Body,
    handler::HandlerWithoutStateExt,
    http::Method,
    response::Redirect,
    routing::{delete, get, post, put},
};
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tokio::time::sleep;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::{debug, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let handle_count = 4;
    debug!("handle_count = {}", handle_count);
    let idle_time = 10;
    debug!("idle_time = {}", idle_time);

    create_output_dir();
    info!("create_output_dir finished");
    prepare_database().await;
    info!("prepare_database finished");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let queue_pool = PgPoolOptions::new()
        .max_connections(handle_count)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    info!("Created and connected to queue pool");

    let mut handles = Vec::new();
    for i in 0..handle_count {
        let pool = queue_pool.clone();
        let handle = tokio::spawn(async move {
            info!("Started worker {i}");
            queue_worker(i, pool).await;
        });
        handles.push(handle);
        sleep(Duration::from_secs((idle_time / handle_count) as u64)).await;
    }
    info!("Started all Workers");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_origin(Any) // Allow any origin
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]) // Specify allowed methods
        .allow_headers([axum::http::header::CONTENT_TYPE]); // Allow specific headers

    let static_files =
        tower_http::services::ServeDir::new("dist").not_found_service(service_fn(fallback_handler));

    let app = Router::new()
        .fallback_service(static_files)
        .route("/test", get(check_system_handler))
        .route("/api/downloadVideo", post(simple_download_handler))
        .route("/api/files/size", post(get_single_dir_size_handler))
        .route("/api/files/dir_delete", delete(dir_delete_handler))
        .route("/api/files", get(list_files))
        .route("/api/yt/mode", get(mode_handler))
        .layer(cors.clone())
        .route("/api/yt/getTitle", post(title_handler))
        .layer(cors.clone())
        .route("/api/files/dir_create", put(create_dir_handler))
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();

    info!("Serving app now");
    axum::serve(listener, app).await.unwrap()
}

async fn fallback_handler(_req: Request<Body>) -> Result<Response, Infallible> {
    let html = fs::read_to_string("dist/index.html")
        .await
        .unwrap_or_else(|_| "<h1>500 - Internal Server Error</h1>".to_string());

    Ok(Response::builder()
        .header("Content-Type", "text/html")
        .body(Body::from(html))
        .unwrap())
}
