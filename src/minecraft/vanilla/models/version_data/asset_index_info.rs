use serde::{Deserialize, Serialize};

use crate::minecraft::vanilla::models::asset_index::AssetIndex;

/// The asset index that needs to be used to get all the needed assets to launch the game.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AssetIndexInfo {
    /// The ID of the index
    pub id: String,
    /// SHA1 of the index JSON
    pub sha1: String,
    /// Size of the index JSON
    pub size: usize,
    /// Size of all the assets contained in the index JSON
    #[serde(alias = "totalSize")]
    pub total_size: i64,
    /// URL of the index JSON
    pub url: String,
}

impl AssetIndexInfo {
    /// Gets the index itself from Minecraft servers.
    pub async fn fetch_index(&self) -> reqwest::Result<AssetIndex> {
        reqwest::get(&self.url)
            .await?
            .error_for_status()?
            .json()
            .await
    }
}
