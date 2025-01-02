#[deny(warnings)]

mod routes;
mod func;

use axum::{
    routing::{delete, get, post}, Router
};
use func::preperations::prepare_database;
use routes::{files::dir_delete_handler, misc::check_system_handler};
use crate::routes::files::{create_dir_handler, get_single_dir_size_handler};
use crate::routes::video::simple_download_handler;
use crate::func::preperations::create_output_dir;


#[tokio::main]
async fn main() {    
    create_output_dir();
    prepare_database().await;

    let app = Router::new()
        .route("/test", get(check_system_handler))
        .route("/api/downloadVideo", post(simple_download_handler))
        .route("/api/files/size", post(get_single_dir_size_handler))
        .route("/api/files/dir_delete", delete(dir_delete_handler))
        .route("/api/files/dir_create", post(create_dir_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap()
}
