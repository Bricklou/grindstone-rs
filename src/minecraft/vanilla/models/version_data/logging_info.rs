use serde::{Deserialize, Serialize};

use super::file::File;

/// Information about the logging configuration
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LoggingInfo {
    /// Logging in the client
    pub client: Client,
}

/// Client logging information
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Client {
    /// The argument passed for logging.
    pub argument: String,
    /// The logging configuration file.
    pub file: File,
    /// Type of the logging file
    #[serde(rename = "type")]
    pub _type: String,
}
