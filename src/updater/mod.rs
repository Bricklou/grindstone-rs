use std::{fs, path::PathBuf};

use crate::{errors::GrindstoneResult, event::EventType, invoke_callback};

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
        invoke_callback!(self, EventType::Starting, "Starting Updater !");

        invoke_callback!(
            self,
            EventType::CreatingFolders,
            "Creating required folders"
        );
        fs::create_dir_all(self.dot_minecraft_path())?;
        fs::create_dir_all(self.updater_folder())?;

        // Check versions on server and download version manifest
        self.save_version_data().await?;

        // Check if a compatible version of java is available
        self.install_java().await?;
        Ok(())
    }
}
