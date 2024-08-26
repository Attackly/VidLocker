use axum::http::StatusCode;
use axum::Json;
use axum_macros::debug_handler;
use crate::routes::responses::DefaultResponse;


#[debug_handler]
/// # check_system_handler
/// THis function will send back information about the App
///
pub async fn check_system_handler() -> Json<DefaultResponse> {
    let resp = DefaultResponse {
        status: StatusCode::OK.as_u16(),
        message: "Up and running".to_string()
    };

    Json(resp)
}