use std::collections::HashMap;

use serde::Deserialize;

use crate::errors::GrindstoneResult;

use super::download_file::DownloadFile;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum FileType {
    Directory,
    File,
    Link,
}

#[derive(Deserialize, Debug)]
pub struct JreFileDownload {
    pub raw: DownloadFile,
}

#[derive(Deserialize, Debug)]
pub struct JreFile {
    #[serde(rename = "type")]
    pub file_type: FileType,
    pub executable: Option<bool>,
    pub downloads: Option<JreFileDownload>,
    pub target: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct JreRuntimeManifest {
    pub files: HashMap<String, JreFile>,
}

impl JreRuntimeManifest {
    /// Get the JRE manifest from Mojang servers.
    pub async fn get<S: Into<String>>(url: S) -> GrindstoneResult<Self> {
        let response = reqwest::Client::new()
            .get(url.into())
            .send()
            .await?
            .error_for_status()?
            .json::<JreRuntimeManifest>()
            .await?;

        Ok(response)
    }
}
