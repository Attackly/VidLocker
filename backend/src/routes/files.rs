use crate::func::files::{dir_create, dir_delete, get_dir_size};
use crate::routes::responses::{DefaultResponse, DirSizeResponse};
use crate::structs::file::FileEntry;
use axum::Json;
use axum::extract::{Path, Query};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
use serde::Deserialize;
use tracing::info;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use urlencoding::decode;

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

pub async fn list_files(Query(params): Query<HashMap<String, String>>) -> Response {
    let base_path = "./output";
    let querried = params.get("dir").map(|d| d.as_str()).unwrap_or("/");
    if querried.contains("..") {
        return (
            StatusCode::BAD_REQUEST,
            Json(DefaultResponse {
                status: StatusCode::BAD_REQUEST.as_u16(),
                message: "Error getting path. Fuck you".to_string(),
            }),
        )
            .into_response();
    }
    // Get the requested directory from query params
    let dir = params.get("dir").map(|d| d.as_str()).unwrap_or("/");
    let full_path = format!("{}/{}", base_path, dir.trim_start_matches('/'));

    let mut files = Vec::new();

    if let Ok(entries) = fs::read_dir(PathBuf::from(&full_path)) {
        for entry in entries.flatten() {
            let metadata = entry.metadata().ok();
            let is_directory = metadata.clone().map(|m| m.is_dir()).unwrap_or(false);
            let name = entry.file_name().into_string().unwrap_or_default();
            let file_size = metadata.map(|m| m.len()).unwrap_or(0);

            files.push(FileEntry {
                name,
                path: format!("{}/{}", dir, entry.file_name().to_string_lossy()),
                is_directory,
                file_size,
            });
        }
    }

    Json(files).into_response()
}

pub async fn download_file_handler(
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    // TODO need to see if its better using just the filename or if i should look for a different way of identifiying the file. Esp. with Authorization in mind. 
    
    let base_path = "./output";
    let file_name = params.get("filename").map(|f|  decode(f.as_str()).unwrap()).unwrap();
    info!("Requested file: {}", file_name);
   
    if file_name.contains("../") || file_name.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            Json(DefaultResponse {
                status: StatusCode::BAD_REQUEST.as_u16(),
                message: "Invalid file path".to_string(),
            }),
        )
            .into_response();
    }
    let full_path = format!("{}/{}", base_path, file_name.trim_start_matches('/'));

    if !PathBuf::from(&full_path).exists() {
        return (
            StatusCode::NOT_FOUND,
            Json(DefaultResponse {
                status: StatusCode::NOT_FOUND.as_u16(),
                message: "File not found".to_string(),
            }),
        )
            .into_response();
    }

    match fs::read(full_path) {
        Ok(contents) => (StatusCode::OK, contents).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(DefaultResponse {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                message: "Error reading file".to_string(),
            }),
        )
            .into_response(),
    }
}

pub async fn delete_file(Path(filename): Path<String>) -> impl IntoResponse {
    fs::remove_file(PathBuf::from(format!("output/{}", filename))).unwrap();
    info!("Deleted file: {}", filename);
    StatusCode::OK.into_response()
}