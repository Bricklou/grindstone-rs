use std::path::PathBuf;

use crate::constants::MC_MS_STORE_IDENTIFIANT;
use crate::errors::GrindstoneResult;

use super::Java;

impl Java {
    #[cfg(windows)]
    pub fn runtime_path(&self) -> GrindstoneResult<PathBuf> {
        // Use MS store first if available
        let mut path = PathBuf::from(dirs::data_local_dir().unwrap())
            .join("Packages").join(MC_MS_STORE_IDENTIFIANT);
        if path.exists() {
            path = path.join("LocalState");
        } else if PathBuf::from("C:/Program Files (x86)/Minecraft").exists() {
            path = PathBuf::from("C:/Program Files (x86)/Minecraft");
        } else {
            path = path.join("LocalState");
        }
        path.push("runtime");
        Ok(path)
    }

    #[cfg(unix)]
    pub fn runtime_path(&self) -> GrindstoneResult<PathBuf> {
        let mut path = self.config.dot_minecraft_path();
        path.push("runtime");
        Ok(path)
    }

    #[cfg(target_os = "macos")]
    /// Not yet implemented, please someone do it, i don't own a Mac OS !
    pub fn runtime_path(&self) -> GrindstoneResult<PathBuf> {
        unimplemented!("I don't own a Mac OS, i can't test my code or where the JRE is located.");
    }

    pub fn search_jre(&self, name: &String) -> GrindstoneResult<Option<PathBuf>> {
        let mut path = self.runtime_path()?;
        path.push(name);

        if !path.exists() {
            return Ok(None);
        }

        Ok(Some(path.to_path_buf()))
    }
}
