use std::path::PathBuf;

use log::{debug, trace};

use crate::{
    config::Config,
    errors::GrindstoneResult,
    event::{EventType, Progress},
    invoke_callback,
    minecraft::java::{jre_manifest::JreManifest, runtime_manifest::JreRuntimeManifest},
};

use super::VersionData;

mod download_file;
mod jre_manifest;
mod runtime_manifest;
mod search_jre;

pub struct Java {
    config: Config,
}

impl Java {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    /// Get java executable from official minecraft JRE
    /// This will download the required JRE if it is not present
    pub async fn install(&self, version_data: VersionData) -> GrindstoneResult<PathBuf> {
        let required_version = version_data.java_version;

        debug!(
            "Version {} require Java JRE {}",
            version_data.id, required_version.major_version
        );

        invoke_callback!(
            &self.config,
            EventType::SearchingForJRE,
            format!(
                "Searching for preinstalled runtime folders {}",
                required_version.component
            )
        );

        let runtime_folder = self.search_jre(&required_version.component)?;

        let java_runtime_path = match runtime_folder {
            Some(folder) => folder,
            None => {
                let runtimes_path = self.runtime_path()?;
                runtimes_path.join(&required_version.component)
            }
        };

        self.download_jre_data(required_version.component, java_runtime_path.clone())
            .await?;

        Ok(java_runtime_path)
    }

    async fn download_jre_data(
        &self,
        name: String,
        java_runtime_path: PathBuf,
    ) -> GrindstoneResult<()> {
        trace!("JRE component name: {}", name);

        let manifest = JreManifest::get().await?;

        cfg_if::cfg_if! {
            if #[cfg(target_os = "linux")]
            {
                if let Some(data) = manifest.linux.get(&name) {
                    let data = data.first().unwrap();
                    let man = JreRuntimeManifest::get(&data.manifest.url).await?;
                    invoke_callback!(self.config, EventType::DownloadJRE(Progress {current: 0, max: 0, message: "".to_string()}), "Downloading JRE");
                    self.download_jre_files(&java_runtime_path, man.files).await?;
                }
            } else if #[cfg(target_os = "windows")]
            {
                if let Some(data) = manifest.windows.get(&name) {
                    let data = data.first().unwrap();
                    let man = JreRuntimeManifest::get(&data.manifest.url).await?;
                    invoke_callback!(self.config, EventType::DownloadJRE(Progress {current: 0, max: 0, message: "".to_string()}), "Downloading JRE");
                    self.download_jre_files(&java_runtime_path, man.files).await?;
                }
            } else if #[cfg(target_os = "macos")]
            {
                if let Some(data) = manifest.mac_os.get(&name) {
                    let data = data.first().unwrap();
                    let man = JreRuntimeManifest::get(&data.manifest.url).await?;

                    invoke_callback!(self.config, EventType::DownloadJRE(Progress {current: 0, max: 0, message: "".to_string()}), "Downloading JRE");
                    self.download_jre_files(&java_runtime_path, man.files).await?;
                }
            } else {
                compile_error!("Unknown platform {}", env::consts::OS)
            }
        }

        Ok(())
    }
}
