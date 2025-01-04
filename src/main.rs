#[deny(warnings)]

mod routes;
mod func;

use axum::{
    routing::{delete, get, post, put}, Router
};
use func::preperations::prepare_database;
use routes::{files::dir_delete_handler, misc::check_system_handler, yt::mode_handler};
use crate::routes::files::{create_dir_handler, get_single_dir_size_handler};
use crate::routes::video::simple_download_handler;
use crate::func::preperations::create_output_dir;
use crate::routes::yt::title_handler;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {    
    create_output_dir();
    prepare_database().await;


    let cors = CorsLayer::new()

    .allow_origin(Any);


    let app = Router::new()
        .route("/test", get(check_system_handler))
        .route("/api/downloadVideo", post(simple_download_handler))
        .route("/api/files/size", post(get_single_dir_size_handler))
        .route("/api/files/dir_delete", delete(dir_delete_handler))
        .route("/api/yt/mode", get(mode_handler))
        .route("/api/yt/getTitle", get(title_handler))
        .route("/api/files/dir_create", put(create_dir_handler))
        .layer(cors);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap()
}
