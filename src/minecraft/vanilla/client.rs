use log::{debug, trace};

use crate::{
    config::Config,
    errors::GrindstoneResult,
    event::EventType,
    invoke_callback,
    minecraft::VersionData,
    utils::download::{download_file_check, Download},
};

pub struct Client;

impl Client {
    pub async fn install(config: &Config, version_data: &VersionData) -> GrindstoneResult<()> {
        let downloads = match &version_data.downloads {
            Some(downloads) => downloads,
            None => {
                debug!(
                    "The version data does not contain download information. Skipping download."
                );
                return Ok(());
            }
        };

        trace!("Building download for client");
        let minecraft_path = config.version_jar_path();
        let sha1 = hex::decode(&downloads.client.sha1)?;
        let downloads = Download {
            url: downloads.client.url.clone(),
            file: minecraft_path,
            sha1: Some(sha1),
        };

        let client = reqwest::Client::new();

        invoke_callback!(config, EventType::DownloadClient, "Dowloading client jar");

        download_file_check(&client, downloads.url, downloads.file, downloads.sha1).await?;

        Ok(())
    }
}
