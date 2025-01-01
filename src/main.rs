#[deny(warnings)]

mod routes;
mod func;

use axum::{
    Router,
    routing::{get, post}
};
use routes::misc::check_system_handler;
use crate::routes::files::{create_dir_handler, get_single_dir_size_handler};
use crate::routes::video::simple_download_handler;
use std::{fs, io};


#[tokio::main]
async fn main() {

    // Launch preperations.
    match fs::create_dir("output") {
        Ok(_) => (),
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
            ();
        }
        Err(e) => panic!("There was an error! {e}")
    }

    let app = Router::new()
        .route("/test", get(check_system_handler))
        .route("/api/downloadVideo", post(simple_download_handler))
        .route("/api/files/size", post(get_single_dir_size_handler))
        .route("/api/files/create", post(create_dir_handler));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap()
}
