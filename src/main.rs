#[deny(warnings)]

mod routes;
mod func;

use axum::{
    Router,
    routing::{get, post}
};
use routes::misc::check_system_handler;
use crate::routes::files::create_dir_handler;
use crate::routes::video::simple_download_handler;

#[tokio::main]
async fn main() {

    let app = Router::new()
        .route("/test", get(check_system_handler))
        .route("/api/downloadVideo", post(simple_download_handler))
        .route("/api/files/create", post(create_dir_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap()
}
