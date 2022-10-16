mod arguments;
pub mod asset_index_info;
mod downloads;
mod extract;
mod file;
pub mod java_version;
pub mod library;
mod library_download;
pub mod logging_info;
mod natives;
mod rules;
pub mod version_type;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use self::{
    arguments::Arguments, asset_index_info::AssetIndexInfo, downloads::Downloads,
    java_version::JavaVersion, library::Library, logging_info::LoggingInfo,
    version_type::VersionType,
};

/// The version data includes all information for installing and launching Minecraft.
/// The version data exists for every Minecraft version (alphas, betas, snapshots, rc, ...).
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VersionData {
    /// Arguments for launching.
    pub arguments: Option<Arguments>,
    #[serde(alias = "assetIndex")]
    /// Asset index information.
    pub asset_index: AssetIndexInfo,
    /// Used asset index.
    pub assets: String,
    /// Compliance level.
    #[serde(alias = "complianceLevel")]
    pub compliance_level: i32,
    /// Downloads of client/server.
    pub downloads: Option<Downloads>,
    /// Version ID.
    pub id: String,
    /// Recommended java version
    #[serde(alias = "javaVersion")]
    pub java_version: JavaVersion,
    /// Libraries.
    pub libraries: Vec<Library>,
    /// Logging information.
    pub logging: Option<LoggingInfo>,
    /// Main class / entry point.
    #[serde(alias = "mainClass")]
    pub main_class: String,
    /// Minecraft arguments.
    /// Used in older versions.
    #[serde(alias = "minecraftArguments")]
    pub minecraft_arguments: Option<String>,
    /// Minimum launcher version.
    /// Applies to the original launcher.
    #[serde(alias = "minimumLauncherVersion")]
    pub minimum_launcher_version: i32,
    /// Release time.
    #[serde(alias = "releaseTime", with = "time::serde::rfc3339")]
    pub release_time: OffsetDateTime,
    /// Release time.
    #[serde(with = "time::serde::rfc3339")]
    pub time: OffsetDateTime,
    /// Type of the version.
    #[serde(rename = "type")]
    pub _type: VersionType,
}

impl VersionData {
    /// Returns all needed libraries by applying the rule of a library.
    pub fn needed_libraries(&self) -> Vec<&Library> {
        self.libraries
            .iter()
            .filter(|library| library.check_use())
            .collect()
    }
}
