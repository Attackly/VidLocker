use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DefaultResponse {
    pub(crate) status: u16,
    pub(crate) message: String
}
