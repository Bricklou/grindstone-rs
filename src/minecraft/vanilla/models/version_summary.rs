use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use super::version_data::version_type::VersionType;

/// Summary of a version that is found in the version manifest
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VersionSummary {
    /// The version ID
    /// Usually this is the version number for releases.
    pub id: String,

    /// The type of version this is.
    /// Release, Snapshot, Beta or Alpha
    #[serde(rename = "type")]
    pub _type: VersionType,

    /// The URL of the version data JSON.
    pub url: String,

    /// Release time
    #[serde(with = "time::serde::rfc3339")]
    pub time: OffsetDateTime,

    /// Release time
    #[serde(alias = "releaseTime", with = "time::serde::rfc3339")]
    pub release_time: OffsetDateTime,

    /// File hash
    pub sha1: String,
}
