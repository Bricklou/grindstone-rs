use serde::{Deserialize, Serialize};

/// The asset index that needs to be used to get all the needed assets to launch the game.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AssetIndex {
    /// The ID of the index
    pub id: String,
    /// SHA1 of the index JSON
    pub sha1: String,
    /// Size of the index JSON
    pub size: usize,
    /// Size of all the assets contained in the index JSON
    #[serde(alias = "totalSize")]
    pub total_size: i64,
    /// Url of the index JSON
    pub url: String,
}
