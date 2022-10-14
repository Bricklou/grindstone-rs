use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    constants,
    errors::{GrindstoneError, GrindstoneResult},
    version::MinecraftVersion,
};

use super::version_summary::VersionSummary;

/// Manifest with all Minecraft versions.
/// Includes identifiers for the latest release and snapshot version.
#[derive(Deserialize, Debug)]
pub struct VersionsManifest {
    /// Latest release and snapshot version.
    pub latest: LatestVersion,
    /// Map of all versions.
    pub versions: HashMap<String, VersionSummary>,
}

impl VersionsManifest {
    /// Get the manifest from Minecraft servers.
    pub async fn fetch() -> GrindstoneResult<Self> {
        let response = reqwest::Client::new()
            .get(constants::MC_VERSION_MANIFEST_URL)
            .send()
            .await?
            .error_for_status()?
            .json::<VersionsManifestResponse>()
            .await?;

        Ok(response.into())
    }

    pub fn get_version(&self, version: &MinecraftVersion) -> GrindstoneResult<&VersionSummary> {
        let summary = self
            .versions
            .get(&version.id)
            .ok_or(GrindstoneError::InvalidVersion(version.id.clone()))?;

        Ok(summary)
    }
}

/// Holds latest release and snapshot version IDs.
#[derive(Debug, Deserialize, Serialize)]
pub struct LatestVersion {
    /// Latest stable release.
    pub release: String,
    /// Latest snapshot release.
    pub snapshot: String,
}

/// Remote manifest file structure provided on Minecraft servers
#[derive(Debug, Deserialize, Serialize)]
struct VersionsManifestResponse {
    pub latest: LatestVersion,
    pub versions: Vec<VersionSummary>,
}

impl From<VersionsManifestResponse> for VersionsManifest {
    fn from(manifest: VersionsManifestResponse) -> Self {
        let mut versions = HashMap::new();

        for version in manifest.versions {
            versions.insert(version.id.clone(), version);
        }

        Self {
            latest: manifest.latest,
            versions,
        }
    }
}
