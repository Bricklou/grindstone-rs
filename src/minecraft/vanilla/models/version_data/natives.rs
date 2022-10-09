use crate::utils::os::Platform;
use serde::{Deserialize, Serialize};

/// Information about available native library.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Natives {
    /// Linux native name.
    pub linux: Option<String>,
    /// Windows native name.
    pub windows: Option<String>,
    /// MacOs native name
    pub osx: Option<String>,
}

impl Natives {
    /// Gets the current platform.
    ///
    /// Returns none when the platform is not supported/implemented.
    /// Currently linux, windows and macos are supported.
    pub fn get_for_current_platform(&self) -> Option<String> {
        let current = Platform::current();

        match current {
            Platform::Linux => self.linux.clone(),
            Platform::Windows => self.windows.clone(),
            Platform::MacOs => self.osx.clone(),
            Platform::Other => None,
        }
    }
}
