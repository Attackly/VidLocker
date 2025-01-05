use axum::Json;
use serde::{Deserialize, Serialize};
use super::responses::ModeResponse;
use crate::func::yt::{get_mode, get_title};
use axum::http::StatusCode;
use crate::func::yt::Video;

#[derive(Serialize, Deserialize)]
pub struct VideoRequest {
    url: String
}

#[derive(Serialize)]
pub struct VideoResponse {
    status: u16,
    video: Option<Video>
}


pub async fn mode_handler() -> Json<ModeResponse> {
    match get_mode().as_str() {
        "api" => Json(ModeResponse { status: StatusCode::OK.as_u16(), mode: "api".to_string() }),
        "fallback" => Json(ModeResponse { status: StatusCode::OK.as_u16(), mode: "fallback".to_string() }),
        _ => Json(ModeResponse { status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(), mode: "unknown".to_string() })
    }
}

#[axum_macros::debug_handler]
pub async fn title_handler(Json(payload): Json<VideoRequest>) -> Json<VideoResponse> {
    if payload.url.len() != 11 {
        return Json(VideoResponse { status: StatusCode::BAD_REQUEST.as_u16(), video: None });
    }
    
    match get_title(&payload.url).await {
        Some(video) => return Json(VideoResponse { status: StatusCode::OK.as_u16(), video: Some(video) }),
        None => {
            return Json(VideoResponse { status: StatusCode::NOT_FOUND.as_u16(), video: None });
        }
    };
}

