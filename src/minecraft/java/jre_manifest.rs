use std::collections::HashMap;

use serde::Deserialize;
use time::OffsetDateTime;

use crate::{constants, errors::GrindstoneResult};

use super::download_file::DownloadFile;

#[derive(Deserialize, Debug)]
pub struct RuntimeVersion {
    pub name: String,
    /// Release time
    #[serde(with = "time::serde::rfc3339")]
    pub released: OffsetDateTime,
}

#[derive(Deserialize, Debug)]
pub struct RuntimeData {
    pub manifest: DownloadFile,
    pub version: RuntimeVersion,
}

#[derive(Deserialize, Debug)]
pub struct JreManifest {
    pub linux: HashMap<String, Vec<RuntimeData>>,
    #[serde(rename = "mac-os")]
    pub mac_os: HashMap<String, Vec<RuntimeData>>,
    #[serde(rename = "mac-os-arm64")]
    pub mac_os_arm64: HashMap<String, Vec<RuntimeData>>,
    #[serde(rename = "windows-x64")]
    pub windows: HashMap<String, Vec<RuntimeData>>,
}

impl JreManifest {
    /// Get the JRE manifest from Mojang servers.
    pub async fn get() -> GrindstoneResult<Self> {
        let response = reqwest::Client::new()
            .get(constants::JAVA_JRE_MANIFEST_URL)
            .send()
            .await?
            .error_for_status()?
            .json::<JreManifest>()
            .await?;

        Ok(response)
    }
}
