use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::Arc,
};

use futures::{stream::FuturesUnordered, StreamExt};
use log::trace;
use serde::{Deserialize, Serialize};
use tokio::{fs, sync::Semaphore};

use crate::{
    config::Config,
    constants,
    errors::GrindstoneResult,
    event::{AssetInstallationUpdate, EventType, Progress},
    invoke_callback,
    utils::download::{download_file_check, Download},
};

/// Map of all the assets for a `Minecraft` version.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AssetIndex {
    /// Wether to install the assets as resources or not.
    ///
    /// Used primarily on older versions (pre1.6).
    #[serde(default)]
    pub map_to_resources: bool,
    /// Asset objects.
    ///
    /// Key is the name of the file.
    pub objects: HashMap<String, AssetInfo>,
}

/// Information of a single asset.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AssetInfo {
    /// SHA1 of the asset file.
    pub hash: String,
    /// Size of the asset file.
    pub size: usize,
}

impl AssetInfo {
    /// Builds the complete path for the asset file.
    pub fn asset_path(&self, assets_path: impl AsRef<Path>) -> PathBuf {
        let mut asset_path = PathBuf::from(assets_path.as_ref());

        asset_path.push("objects");
        asset_path.push(self.hash.chars().take(2).collect::<String>().as_str());
        asset_path.push(&self.hash);

        asset_path
    }

    /// Builds the complete path for the asset file mapped as a resource.
    pub fn resource_path(key: &str, minecraft_path: impl AsRef<Path>) -> PathBuf {
        let mut resource_path = PathBuf::from(minecraft_path.as_ref());

        resource_path.push("resources");
        resource_path.push(key);

        resource_path
    }

    /// Builds the download URL for the asset.
    pub fn download_url(&self) -> String {
        let mut url = vec![constants::MC_ASSETS_BASE_URL];

        let part = self.hash.chars().take(2).collect::<String>();

        url.push(part.as_str());
        url.push(&self.hash);

        url.join("/")
    }
}

impl AssetIndex {
    pub async fn install_assets(&self, config: &Config) -> GrindstoneResult<()> {
        let downloads = self
            .objects
            .values()
            .map(|a| build_download(a, config.assets_path()))
            .collect::<Result<Vec<_>, _>>()?;

        let mut count = 0;
        let max = downloads.len() as u32;

        let log_progress = |cur: u32, m: u32, msg: String| {
            invoke_callback!(
                config,
                EventType::Assets(
                    Progress {
                        current: cur,
                        max: m,
                        message: msg
                    },
                    AssetInstallationUpdate::Downloading
                ),
                "Downloading assets"
            );
        };

        let client = reqwest::Client::new();
        let mut tasks = FuturesUnordered::new();
        let semaphore = Arc::new(Semaphore::new(constants::MAX_PARALLEL_DOWNLOAD));

        for d in downloads {
            let c = client.clone();

            let permit = semaphore.clone().acquire_owned().await.unwrap();
            tasks.push(tokio::spawn(async move {
                let res = download_file_check(&c, d.url, d.file, d.sha1).await;
                drop(permit);
                res
            }));
        }

        while let Some(Ok(res)) = tasks.next().await {
            count += 1;
            let name = res?;
            log_progress(count, max, format!("Downloaded asset {}", name));
        }

        if self.map_to_resources {
            let mut tasks = FuturesUnordered::new();

            for (key, asset) in self.objects.iter() {
                let a = create_symlink(key, asset, config);
                tasks.push(a);
            }

            while let Some(res) = tasks.next().await {
                count += 1;
                let name = res?;
                log_progress(count, max, format!("Symlinked asset {}", name));
            }
        }

        Ok(())
    }
}

fn build_download(asset: &AssetInfo, assets_path: impl AsRef<Path>) -> GrindstoneResult<Download> {
    let sha1 = hex::decode(&asset.hash)?;

    Ok(Download {
        url: asset.download_url(),
        file: asset.asset_path(assets_path),
        sha1: Some(sha1),
    })
}

async fn create_symlink(
    key: &String,
    asset: &AssetInfo,
    config: &Config,
) -> GrindstoneResult<String> {
    let assets_path = config.assets_path();
    let minecraft_path = config.dot_minecraft_path();

    let name = asset
        .asset_path(&assets_path)
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_default();

    let asset_path = asset.asset_path(&assets_path);
    let resource_path = AssetInfo::resource_path(key.as_str(), &minecraft_path);
    let parent_dir = resource_path.parent().unwrap();

    trace!("Creating parent directory for symlink");
    fs::create_dir_all(parent_dir).await?;

    trace!(
        "Creating symlink: {} => {}",
        resource_path.to_string_lossy(),
        asset_path.to_string_lossy()
    );

    #[cfg(unix)]
    fs::symlink(asset_path, resource_path).await?;

    #[cfg(windows)]
    fs::symlink_file(asset_path, resource_path).await?;

    Ok(name)
}
