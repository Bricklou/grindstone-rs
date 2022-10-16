use std::path::PathBuf;

use log::{debug, trace};

use crate::{
    config::Config,
    errors::GrindstoneResult,
    event::EventType,
    invoke_callback,
    minecraft::VersionData,
    utils::download::{download_file_check, Download},
};

use super::models::version_data::logging_info::LoggingInfo;

impl LoggingInfo {
    pub async fn install_log_patch(
        config: &Config,
        version_data: &VersionData,
    ) -> GrindstoneResult<()> {
        let logging_info = match &version_data.logging {
            Some(info) => info,
            None => {
                debug!("The version data doesn't contain logging information. Skipping download.");
                return Ok(());
            }
        };

        trace!("Building download for log config");
        let mut config_path = config.log_configs_path();
        config_path.push(
            logging_info
                .client
                .file
                .id
                .as_ref()
                .expect("Logging Info has no ID"),
        );

        let sha1 = hex::decode(&logging_info.client.file.sha1)?;
        let downloads = Download {
            url: logging_info.client.file.url.clone(),
            file: config_path,
            sha1: Some(sha1),
        };

        let client = reqwest::Client::new();

        invoke_callback!(
            config,
            EventType::DownloadLogConfig,
            "Downloading log config"
        );

        download_file_check(&client, downloads.url, downloads.file, downloads.sha1).await?;

        Ok(())
    }
}
