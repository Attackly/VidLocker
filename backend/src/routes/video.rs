use axum::extract::State;
use crate::func::video::{download_video_simple_ydl, write_db_entry};
use crate::routes::responses::DefaultResponse;
use axum::http::StatusCode;
use axum::Json;
use axum_macros::debug_handler;
use serde::Deserialize;
use url::Url;
#[derive(Deserialize)]
pub struct VideoRequest {
    url: String,
    path: Option<String>
}

#[debug_handler]
pub async fn simple_download_handler(
    State(state): State<crate::AppState>,
    Json(payload): Json<VideoRequest>
) -> Json<DefaultResponse> {
    if payload.url.len() != 11 {
        match Url::parse(&payload.url) {
            Ok(_) => {}
            Err(_) => {
                return Json(DefaultResponse {
                    status: StatusCode::BAD_REQUEST.as_u16(),
                    message: "The Url is invalid".to_string(),
                });
            }
        }
    }

    tokio::spawn(async move {
        let path = payload.path.unwrap_or_else(|| {
            let default_path = "shared".to_string();
            default_path
        });
        if std::env::var("MODE").unwrap_or("direct".to_string()) == "queue" {
            write_db_entry(&payload.url, state).await;
        } else {
            download_video_simple_ydl(payload.url, path).await;
        }
    });

    Json(DefaultResponse {
        status: 200,
        message: "Started downloading Video".to_string(),
    })
}
