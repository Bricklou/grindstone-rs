use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::file::File;

/// Download information of a library.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LibraryDownloads {
    /// Library file information.
    pub artifact: Option<File>,
    /// Library classifiers for natives.
    #[serde(default)]
    pub classifiers: HashMap<String, File>,
}
