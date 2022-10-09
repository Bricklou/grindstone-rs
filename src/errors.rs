/// Custom Result type with [`GrindstoneError`](GrindstoneError) as Error type
pub type GrindstoneResult<T> = Result<T, GrindstoneError>;

#[derive(Debug, thiserror::Error)]
/// Custom Error type for the whole library
pub enum GrindstoneError {
    #[error("Invalid configuration `{0}`")]
    InvalidConfig(String),

    /// Standard IO Error.
    #[error("{0}")]
    IO(std::io::Error),

    /// Problem while performing a web request.
    /// Web requests are used to get game information and resources.
    #[error("{0}")]
    Reqwest(reqwest::Error),

    /// Serializing or deserializing of some data failed.
    #[error("{0}")]
    Serde(serde_json::Error),

    /// Problem while comparing checksums of files.
    /// Occurs mostly when downloading resources.
    #[error("Checksums do not match")]
    ChecksumMismatch,

    /// A checksum that is provided from an index has an invalid format.
    /// Versiondata and the Assetindex provide checksums for resources.
    #[error("Checksum does not have a valid format: {0}")]
    InvalidChecksum(hex::FromHexError),

    /// A provided version could not be found in the version manifest.
    /// [Version Manifest](https://piston-meta.mojang.com/mc/game/version_manifest_v2.json)
    #[error("Minecraft version '{0}' is invalid")]
    InvalidVersion(String),

    /// Problem occured while parsing minecraft library name.
    /// Format: `<package>:<name>:<version>`
    #[error("Format of a library name is invalid and not supported")]
    LibraryNameFormat,
}

impl From<std::io::Error> for GrindstoneError {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err)
    }
}

impl From<serde_json::Error> for GrindstoneError {
    fn from(err: serde_json::Error) -> Self {
        Self::Serde(err)
    }
}

impl From<reqwest::Error> for GrindstoneError {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}

impl From<hex::FromHexError> for GrindstoneError {
    fn from(err: hex::FromHexError) -> Self {
        Self::InvalidChecksum(err)
    }
}
