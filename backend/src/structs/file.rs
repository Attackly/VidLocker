use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    is_directory: bool,
}
