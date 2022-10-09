use serde::{Deserialize, Serialize};

/// Additional information about extraction of libraries.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Extract {
    /// Files to exclude when extracting
    #[serde(default)]
    pub exclude: Vec<String>,
}
