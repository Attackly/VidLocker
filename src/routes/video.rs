use axum::http::StatusCode;
use axum::Json;
use axum_macros::debug_handler;
use serde::Deserialize;
use url::Url;
use crate::func::video::download_video_simple_ydl;
use crate::routes::responses::DefaultResponse;

#[derive(Deserialize)]
pub struct VideoRequest {
    url: String
}

#[debug_handler]
pub async fn simple_download_handler(Json(payload): Json<VideoRequest>) -> Json<DefaultResponse> {
    match Url::parse(&payload.url) {
        Ok(_) => {}
        Err(_) => {
            return Json(DefaultResponse{status: StatusCode::BAD_REQUEST.as_u16(), message: "The Url is invalid".to_string()});
        }
    }

    download_video_simple_ydl(payload.url);

    Json(DefaultResponse {
        status: 200,
        message: "Started downloading Video".to_string()
    })
}