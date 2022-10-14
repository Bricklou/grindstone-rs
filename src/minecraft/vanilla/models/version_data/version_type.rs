use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// The type of version.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum VersionType {
    /// Stable release.
    #[serde(alias = "release")]
    Release,
    /// Snapshot or pre-release.
    #[serde(alias = "snapshot")]
    Snapshot,
    /// Old alpha.
    #[serde(alias = "old_alpha")]
    OldAlpha,
    /// Old beta.
    #[serde(alias = "old_beta")]
    OldBeta,
}

impl Display for VersionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VersionType::Release => write!(f, "Release"),
            VersionType::Snapshot => write!(f, "Snapshot"),
            VersionType::OldAlpha => write!(f, "Alpha"),
            VersionType::OldBeta => write!(f, "Beta"),
        }
    }
}
