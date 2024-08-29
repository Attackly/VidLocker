use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DefaultResponse {
    pub(crate) status: u16,
    pub(crate) message: String
}

#[derive(Serialize)]
pub struct DirSizeResponse {
    pub(crate) size: u64
}