use super::responses::ModeResponse;
use crate::func::yt::VideoResp;
use crate::func::yt::{get_mode, get_title};
use axum::Json;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct VideoRequest {
    url: String,
}

#[derive(Serialize)]
pub struct VideoResponse {
    status: u16,
    video: Option<VideoResp>,
}

pub async fn mode_handler() -> Json<ModeResponse> {
    match get_mode().as_str() {
        "api" => Json(ModeResponse {
            status: StatusCode::OK.as_u16(),
            mode: "api".to_string(),
        }),
        "fallback" => Json(ModeResponse {
            status: StatusCode::OK.as_u16(),
            mode: "fallback".to_string(),
        }),
        _ => Json(ModeResponse {
            status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            mode: "unknown".to_string(),
        }),
    }
}

#[axum_macros::debug_handler]
pub async fn title_handler(Json(payload): Json<VideoRequest>) -> Json<VideoResponse> {
    match get_title(&payload.url).await {
        Some(video) => {
            return Json(VideoResponse {
                status: StatusCode::OK.as_u16(),
                video: Some(video),
            });
        }
        None => {
            return Json(VideoResponse {
                status: StatusCode::NOT_FOUND.as_u16(),
                video: None,
            });
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;
    use std::env;

    #[tokio::test]
    #[serial]
    async fn test_mode_handler_api() {
        // TODO this scares me
        unsafe {
            env::set_var("YT_API_KEY", "A_FAKE_KEY");
        }
        let response = mode_handler().await;
        assert_eq!(
            response.0,
            ModeResponse {
                status: 200,
                mode: "api".to_string()
            }
        );
    }

    #[tokio::test]
    #[serial]
    async fn test_mode_handler_fallback() {
        // TODO this scares me
        unsafe {
            env::remove_var("YT_API_KEY");
        }
        let response = mode_handler().await;
        assert_eq!(
            response.0,
            ModeResponse {
                status: 200,
                mode: "fallback".to_string()
            }
        );
    }
}
