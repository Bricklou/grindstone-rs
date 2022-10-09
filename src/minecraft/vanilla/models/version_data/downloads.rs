use serde::{Deserialize, Serialize};

use super::file::File;

/// Contains information about downloading the main client/server file.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Downloads {
    /// Client file
    pub client: File,
    /// Client mappings
    pub client_mappings: Option<File>,
    /// Server file
    pub server: Option<File>,
    /// Server mappings
    pub server_mappings: Option<File>,
}
