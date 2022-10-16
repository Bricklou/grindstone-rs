use std::{fs, path::PathBuf};

use crate::{
    errors::GrindstoneResult,
    event::EventType,
    invoke_callback,
    minecraft::{java::Java, Library, LoggingInfo, VersionData, VersionsManifest},
};

use self::config::Config;

pub mod config;
pub mod event;
mod paths;
pub mod version;

pub struct GrindstoneUpdater {
    pub config: Config,
    pub java_runtime_path: PathBuf,
}

impl GrindstoneUpdater {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            java_runtime_path: PathBuf::new(),
        }
    }

    pub async fn update(&mut self) -> GrindstoneResult<()> {
        invoke_callback!(self.config, EventType::Starting, "Starting Updater !");

        invoke_callback!(
            self.config,
            EventType::CreatingFolders,
            "Creating required folders"
        );
        fs::create_dir_all(self.config.dot_minecraft_path())?;
        fs::create_dir_all(self.config.updater_folder())?;

        // Check versions on server and download version manifest
        invoke_callback!(
            &self.config,
            crate::event::EventType::DownloadManifest,
            "Downloading version manifest"
        );
        let manifest = VersionsManifest::fetch().await?;

        // The dev asked for "latest" version, then change it to the latest available
        // game version in the manifest
        if self.config.version.id == "latest" {
            self.config.version.id = manifest.latest.release.clone();
        }

        // Figure out version
        let summary = manifest.get_version(&self.config.version)?;

        // Fetch version data
        let v_data = VersionData::fetch(&summary.url).await?;
        // Save version data
        v_data.save(&self.config).await?;

        // Check if a compatible version of java is available
        let java = Java::new(self.config.clone());
        self.java_runtime_path = java.install(v_data.clone()).await?;

        // Download libraries
        Library::install_libraries(&self.config, v_data.clone()).await?;

        // Download asset index
        invoke_callback!(
            &self.config,
            crate::event::EventType::DownloadAssetIndex,
            "Downloading assets index"
        );
        let asset_index = v_data.asset_index.fetch_index().await?;

        asset_index.install_assets(&self.config).await?;

        LoggingInfo::install_log_patch(&self.config, &v_data).await?;

        Ok(())
    }

    pub fn launch() -> GrindstoneResult<()> {
        Ok(())
    }
}
