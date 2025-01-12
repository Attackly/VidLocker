use axum::Json;
use axum::response::IntoResponse;
use axum::response::Response;
use serde::Deserialize;
use crate::func::files::{dir_create, get_dir_size, dir_delete};
use crate::routes::responses::{DefaultResponse, DirSizeResponse};
use axum::http::StatusCode;


#[derive(Deserialize)]
pub struct PathRequest {
    path: String
}

pub async fn create_dir_handler(Json(payload): Json<PathRequest>) -> Response {

    match dir_create(payload.path).await {
        Ok(_) => {
            (StatusCode::OK, Json(DefaultResponse{ status: 200, message: "Path has been created".to_string()})).into_response()
        }
        Err(1) => {
            (StatusCode::BAD_REQUEST, Json(DefaultResponse{status:403, message: "Illegal Path".to_string()})).into_response()
        }
        Err(2) => {
            (StatusCode::BAD_REQUEST, Json(DefaultResponse{status:403, message: "Error Creating Path".to_string()})).into_response()
        }
        _ => {
            (StatusCode::BAD_REQUEST, Json(DefaultResponse{status:500, message: "This error cant appear".to_string()})).into_response()
        }
    }
}

/// # get_single_dir_size_handler
/// This function will return the size of a directory
pub async fn get_single_dir_size_handler(Json(payload): Json<PathRequest>) -> Response {
    match get_dir_size(payload.path).await {
        None => {
            (StatusCode::BAD_REQUEST, Json(DefaultResponse{status:400, message: "Dir does not exist or is invalid or offlimits for you".to_string()})).into_response()
        }
        Some(size) => {
            (StatusCode::OK, Json(DirSizeResponse{size})).into_response()
        }
    }
}

pub async fn dir_delete_handler(Json(payload): Json<PathRequest>) -> Response {
    match dir_delete(payload.path).await {
        Ok(_) => {
            (StatusCode::OK, Json(DefaultResponse{status:200, message: "Dir deleted".to_string()})).into_response()
        }
        Err(1) => {
            (StatusCode::BAD_REQUEST, Json(DefaultResponse{status:403, message: "Illegal Path".to_string()})).into_response()
        }
        Err(2) => {
            (StatusCode::BAD_REQUEST, Json(DefaultResponse{status:403, message: "Error Deleting Path".to_string()})).into_response()
        }
        _ => {
            (StatusCode::BAD_REQUEST, Json(DefaultResponse{status:500, message: "This error should be impossible to appear. If it does. Please open an Issue.".to_string()})).into_response()
        }
    }
}