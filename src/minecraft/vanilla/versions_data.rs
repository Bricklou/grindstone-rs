use std::fs;

use log::trace;
use tokio::{fs::File, io::AsyncWriteExt};

use crate::{config::Config, errors::GrindstoneResult};

use super::models::version_data::VersionData;

impl VersionData {
    /// Fetch the version data JSON
    pub async fn fetch(url: &String) -> reqwest::Result<Self> {
        reqwest::get(url)
            .await?
            .error_for_status()?
            .json::<Self>()
            .await
    }

    /// Saves the version data JSON to disk
    pub async fn save(&self, config: &Config) -> GrindstoneResult<()> {
        // Save version data
        let version_data_path = config.version_data_path();

        let version_data_json = serde_json::to_vec_pretty(self)?;

        if let Some(parent) = version_data_path.parent() {
            trace!("Creating parent folder for version data JSON");
            std::fs::create_dir_all(parent)?;
        }

        let mut file = File::create(version_data_path).await?;
        file.write_all(&version_data_json).await?;
        file.sync_all().await?;

        Ok(())
    }

    /// Read the version data JSON from disk.
    /// This does not download the index if it does not exist.
    pub fn read_version_data(config: &Config) -> GrindstoneResult<VersionData> {
        let version_data_path = config.version_data_path();

        let version_data_file = fs::File::open(version_data_path)?;
        let version_data = serde_json::from_reader::<_, VersionData>(version_data_file)?;

        Ok(version_data)
    }
}
