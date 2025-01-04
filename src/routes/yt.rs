use axum::Json;
use serde::{Deserialize, Serialize};
use super::responses::ModeResponse;
use crate::func::yt::{get_mode, get_title};
use axum::http::StatusCode;


#[derive(Serialize, Deserialize)]
pub struct VideoRequest {
    url: String
}


pub async fn mode_handler() -> Json<ModeResponse> {
    match get_mode().as_str() {
        "api" => Json(ModeResponse { status: StatusCode::OK.as_u16(), mode: "api".to_string() }),
        "fallback" => Json(ModeResponse { status: StatusCode::OK.as_u16(), mode: "fallback".to_string() }),
        _ => Json(ModeResponse { status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(), mode: "unknown".to_string() })
    }
}

#[axum_macros::debug_handler]
pub async fn title_handler(Json(payload): Json<VideoRequest>) -> Json<ModeResponse> {
    get_title(&payload.url).await;

    Json(ModeResponse { status: StatusCode::OK.as_u16(), mode: "api".to_string() })
}

