// #[deny(warnings)]
mod func;
mod queue;
mod routes;
mod structs;
use sqlx::PgPool;
use crate::{
    func::preperations::{create_output_dir, prepare_database},
    queue::queue_worker::queue_worker,
    routes::{
        files::{create_dir_handler, dir_delete_handler, get_single_dir_size_handler, list_files, delete_file, dir_post_handler},
        misc::check_system_handler,
        video::simple_download_handler,
        yt::{mode_handler, title_handler},
    },
};
use axum_server::tls_rustls::RustlsConfig;
use axum::response::Response;
use routes::files::download_file_handler;
use std::{convert::Infallible, net::SocketAddr};
use tokio::fs;
use tower::service_fn;

use axum::{Router, body::Body, http::Request, routing::{delete, get, post, put}};
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tokio::time::sleep;
use tower_http::trace::TraceLayer;
use tracing::{debug, error, info, Level};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;

#[derive(Clone)]
struct AppState {
    db_pool: PgPool,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::builder()
                                 .with_default_directive(LevelFilter::WARN.into())
                                 .from_env_lossy())
        .json()
        .with_current_span(true)
        .with_span_list(true)
        .init();

    let handle_count = 1;
    debug!("handle_count = {}", handle_count);
    let idle_time = 60;
    debug!("idle_time = {}", idle_time);

    create_output_dir().await;
    info!("create_output_dir finished");
    prepare_database().await;
    info!("prepare_database finished");


    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create database pool");

    let app_state = AppState { db_pool: pool };

    let queue_pool = PgPoolOptions::new()
        .max_connections(handle_count)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    info!("Created and connected to queue pool");

    tokio::spawn(async move {
        let mut handles = Vec::new();
    for i in 0..handle_count {
        let pool = queue_pool.clone();
        let handle = tokio::spawn(async move {
            info!("Started worker {i}");
            queue_worker(i, pool, idle_time).await;
        });
        handles.push(handle);
        sleep(Duration::from_secs(idle_time / handle_count as u64)).await;
    }
    info!("Started all Workers");

    });
    
   // let cors = CorsLayer::new()
   //     .allow_origin(Any)
   //     .allow_origin(Any)
   //     .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
   //     .allow_headers([axum::http::header::CONTENT_TYPE]);

    let static_files =
        tower_http::services::ServeDir::new("dist").not_found_service(service_fn(fallback_handler));

    info!("Create Static file Server");
    
    let app = Router::new()
        .layer(TraceLayer::new_for_http().make_span_with(tower_http::trace::DefaultMakeSpan::new().level(Level::INFO)).on_response(tower_http::trace::DefaultOnResponse::new().level(Level::INFO)))
        .fallback_service(static_files)
        .route("/test", get(check_system_handler))
        .route("/api/downloadVideo", post(simple_download_handler))
        .route("/api/files/size", post(get_single_dir_size_handler))
        .route("/api/files/directroy", delete(dir_delete_handler).post(dir_post_handler))
        .route("/api/files", get(list_files))
        .route("/api/file/{*filename}", delete(delete_file))
        .route("/api/files/download", get(download_file_handler))
        .route("/api/yt/mode", get(mode_handler))
        .route("/api/yt/getTitle", post(title_handler))
        .route("/api/files/dir_create", put(create_dir_handler)).with_state(app_state);
    
    
    let sslconfig = RustlsConfig::from_pem_file(
        "certs/cert.crt",
        "certs/cert.key",
    ).await;
    
    if sslconfig.is_err() {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
        error!("Failed to load SSL configuration: {:?} Falling back to HTTP", sslconfig.err());
        info!("Serving app on port 3001: http://localhost:3001");
        axum::serve(listener, app).await.unwrap()
    }
    else {
        let socket_addr = SocketAddr::from(([0, 0, 0, 0], 3001));
        info!("Serving app with SSL on port 3001: https://localhost:3001");
        axum_server::bind_rustls(socket_addr, sslconfig.unwrap()).serve(app.into_make_service()).await.unwrap();
    }
}

//noinspection ALL
async fn fallback_handler(_req: Request<Body>) -> Result<Response, Infallible> {
    let html = fs::read_to_string("dist/index.html")
        .await
        .unwrap_or_else(|e| format!("<h1>500 - Internal Server Error: {:?}</h1>", e));

    Ok(Response::builder()
        .header("Content-Type", "text/html")
        .body(Body::from(html))
        .unwrap())
}
