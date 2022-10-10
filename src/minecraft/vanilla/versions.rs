use std::fs;

use crate::{
    errors::{GrindstoneError, GrindstoneResult},
    invoke_callback,
    minecraft::vanilla::models::version_manifest::VersionsManifest,
    utils::download::download_file_check,
    GrindstoneUpdater,
};

use super::models::version_data::VersionData;

impl GrindstoneUpdater {
    /// Saves the version data JSON to disk
    pub async fn save_version_data(&self) -> GrindstoneResult<()> {
        invoke_callback!(
            self,
            crate::event::EventType::DownloadManifest,
            "Downloading version manifest"
        );

        let manifest = VersionsManifest::get().await?;

        let mut version_id = self.config.version.id.clone();
        if version_id == "latest" {
            version_id = manifest.latest.release;
        }

        // Figure out version
        let version_summary = manifest
            .versions
            .get(&version_id)
            .ok_or(GrindstoneError::InvalidVersion(version_id))?;

        // Save version data
        let version_data_path = self.version_data_path();
        std::fs::create_dir_all(version_data_path.parent().unwrap())?;

        let version_hash = hex::decode(&version_summary.sha1)?;
        download_file_check(&version_summary.url, &version_data_path, Some(version_hash)).await?;

        Ok(())
    }

    /// Read the version data JSON from disk.
    /// This does not download the index if it does not exist.
    pub fn read_version_data(&self) -> GrindstoneResult<VersionData> {
        let version_data_path = self.version_data_path();

        let version_data_file = fs::File::open(version_data_path)?;
        let version_data = serde_json::from_reader::<_, VersionData>(version_data_file)?;

        Ok(version_data)
    }
}
