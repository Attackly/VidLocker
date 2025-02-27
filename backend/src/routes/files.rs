use crate::func::files::{dir_create, dir_delete, get_dir_size};
use crate::routes::responses::{DefaultResponse, DirSizeResponse};
use crate::structs::file::FileEntry;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
#[derive(Deserialize)]
pub struct PathRequest {
    path: String,
}

pub async fn create_dir_handler(Json(payload): Json<PathRequest>) -> Response {
    match dir_create(payload.path).await {
        Ok(_) => (
            StatusCode::OK,
            Json(DefaultResponse {
                status: 200,
                message: "Path has been created".to_string(),
            }),
        )
            .into_response(),
        Err(1) => (
            StatusCode::BAD_REQUEST,
            Json(DefaultResponse {
                status: 403,
                message: "Illegal Path".to_string(),
            }),
        )
            .into_response(),
        Err(2) => (
            StatusCode::BAD_REQUEST,
            Json(DefaultResponse {
                status: 403,
                message: "Error Creating Path".to_string(),
            }),
        )
            .into_response(),
        _ => (
            StatusCode::BAD_REQUEST,
            Json(DefaultResponse {
                status: 500,
                message: "This error cant appear".to_string(),
            }),
        )
            .into_response(),
    }
}

/// # get_single_dir_size_handler
/// This function will return the size of a directory
pub async fn get_single_dir_size_handler(Json(payload): Json<PathRequest>) -> Response {
    match get_dir_size(payload.path).await {
        None => (
            StatusCode::BAD_REQUEST,
            Json(DefaultResponse {
                status: 400,
                message: "Dir does not exist or is invalid or offlimits for you".to_string(),
            }),
        )
            .into_response(),
        Some(size) => (StatusCode::OK, Json(DirSizeResponse { size })).into_response(),
    }
}

pub async fn dir_delete_handler(Json(payload): Json<PathRequest>) -> Response {
    match dir_delete(payload.path).await {
        Ok(_) => (
            StatusCode::OK,
            Json(DefaultResponse {
                status: 200,
                message: "Dir deleted".to_string(),
            }),
        )
            .into_response(),
        Err(1) => (
            StatusCode::BAD_REQUEST,
            Json(DefaultResponse {
                status: 403,
                message: "Illegal Path".to_string(),
            }),
        )
            .into_response(),
        Err(2) => (
            StatusCode::BAD_REQUEST,
            Json(DefaultResponse {
                status: 403,
                message: "Error Deleting Path".to_string(),
            }),
        )
            .into_response(),
        _ => (
            StatusCode::BAD_REQUEST,
            Json(DefaultResponse {
                status: 500,
                message:
                    "This error should be impossible to appear. If it does. Please open an Issue."
                        .to_string(),
            }),
        )
            .into_response(),
    }
}

async fn list_files(Query(params): Query<HashMap<String, String>>) -> Json<Vec<FileEntry>> {
    let base_path = "./uploads"; // Change this to your file directory

    // Get the requested directory from query params
    let dir = params.get("dir").map(|d| d.as_str()).unwrap_or("/");
    let full_path = format!("{}/{}", base_path, dir.trim_start_matches('/'));

    let mut files = Vec::new();

    if let Ok(entries) = fs::read_dir(PathBuf::from(&full_path)) {
        for entry in entries.flatten() {
            let metadata = entry.metadata().ok();
            let is_directory = metadata.map(|m| m.is_dir()).unwrap_or(false);
            let name = entry.file_name().into_string().unwrap_or_default();

            files.push(FileEntry {
                name,
                path: format!("{}/{}", dir, entry.file_name().to_string_lossy()),
                is_directory,
            });
        }
    }

    Json(files)
}
