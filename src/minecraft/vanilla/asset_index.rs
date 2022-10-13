use crate::{
    config::Config,
    errors::GrindstoneResult,
    event::EventType,
    invoke_callback,
    minecraft::{AssetIndex, VersionData},
    utils::download::download_file_check,
};

impl AssetIndex {
    /// Saves the asset index JSON to disk.
    pub async fn save(config: &Config, version_data: VersionData) -> GrindstoneResult<()> {
        if let Some(asset_index) = &version_data.asset_index {
            let mut indexes_path = config.asset_index_path();

            // Create folder
            std::fs::create_dir_all(&indexes_path)?;

            // Download file
            indexes_path.push(format!("{}.json", &version_data.assets));
            let sha = hex::decode(&asset_index.sha1)?;
            invoke_callback!(
                config,
                EventType::DownloadAssetIndex,
                format!("Downloading asset index for {}", &version_data.id)
            );
            let client = reqwest::Client::new();
            download_file_check(&client, &asset_index.url, indexes_path, Some(sha)).await?;
        }

        Ok(())
    }
}
