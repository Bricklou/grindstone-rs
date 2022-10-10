use log::{debug, trace};

use crate::{
    errors::GrindstoneResult,
    event::{EventType, Progress},
    invoke_callback,
    minecraft::java::{jre_manifest::JreManifest, runtime_manifest::JreRuntimeManifest},
    GrindstoneUpdater,
};

mod download_file;
mod jre_manifest;
mod runtime_manifest;
mod search_jre;

impl GrindstoneUpdater {
    /// Get java executable from official minecraft JRE
    /// This will download the required JRE if it is not present
    pub async fn install_java(&mut self) -> GrindstoneResult<()> {
        let data = self.read_version_data()?;

        let required_version = data.java_version;

        debug!(
            "Version {} require Java JRE {}",
            data.id, required_version.major_version
        );

        self.download_jre_data(required_version.component).await?;

        Ok(())
    }

    async fn download_jre_data(&mut self, name: String) -> GrindstoneResult<()> {
        trace!("JRE component name: {}", name);

        let manifest = JreManifest::get().await?;

        invoke_callback!(
            self,
            EventType::SearchingForJRE,
            format!("Searching for preinstalled runtime folders {}", name)
        );
        let runtime_folder = self.search_jre(&name)?;

        if let Some(folder) = runtime_folder {
            self.java_runtime_path = folder;
        } else {
            let runtimes_path = self.runtime_path()?;
            self.java_runtime_path = runtimes_path.join(&name)
        }

        cfg_if::cfg_if! {
            if #[cfg(target_os = "linux")]
            {
                if let Some(data) = manifest.linux.get(&name) {
                    let data = data.first().unwrap();
                    let man = JreRuntimeManifest::get(&data.manifest.url).await?;
                    invoke_callback!(self, EventType::DownloadJRE(Progress {current: 0, max: 0, message: "".to_string()}), "Downloading JRE");
                    self.download_jre_files(self.java_runtime_path.clone(), man.files).await?;
                }
            } else if #[cfg(target_os = "windows")]
            {
                if cfg!(target_pointer_width = "64") {
                    if let Some(data) = manifest.windows.get(&name) {
                        let data = data.first().unwrap();
                    let man = JreRuntimeManifest::get(&data.manifest.url).await?;
                    invoke_callback!(self, EventType::DownloadJRE(Progress {current: 0, max: 0, message: "".to_string()}), "Downloading JRE");
                    self.download_jre_files(self.java_runtime_path.clone(), man.files).await?;
                    }
                } else {
                    if let Some(data) = manifest.windows_32.get(&name) {
                        let data = data.first().unwrap();
                    let man = JreRuntimeManifest::get(&data.manifest.url).await?;

                    invoke_callback!(self, EventType::DownloadJRE(Progress {current: 0, max: 0, message: "".to_string()}), "Downloading JRE");
                    self.download_jre_files(self.java_runtime_path.clone(), man.files).await?;
                    }
                }
            } else if #[cfg(target_os = "macos")]
            {
                if let Some(data) = manifest.mac_os.get(&name) {
                    let data = data.first().unwrap();
                    let man = JreRuntimeManifest::get(&data.manifest.url).await?;

                    invoke_callback!(self, EventType::DownloadJRE(Progress {current: 0, max: 0, message: "".to_string()}), "Downloading JRE");
                    self.download_jre_files(self.java_runtime_path.clone(), man.files).await?;
                }
            } else {
                compile_error!("Unknow platform {}", env::consts::OS)
            }
        }

        Ok(())
    }
}
