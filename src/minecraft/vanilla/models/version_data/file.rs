use serde::{Deserialize, Serialize};

/// Information about a downloadable file.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct File {
    /// ID of the file
    pub id: Option<String>,
    /// Path of the file
    pub path: Option<String>,
    /// SHA1 of the file
    pub sha1: String,
    /// Size of the file
    pub size: usize,
    /// Url of the file
    pub url: String,
}
