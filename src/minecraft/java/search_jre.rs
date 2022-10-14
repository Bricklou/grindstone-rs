use std::path::PathBuf;

use crate::errors::GrindstoneResult;

use super::Java;

impl Java {
    #[cfg(windows)]
    pub fn runtime_path(&self) -> GrindstoneResult<PathBuf> {
        // Use MS store first if available
        let mut path = PathBuf::from(dirs::data_local_dir().unwrap())
            .join("Packages")
            .join("Microsoft.4297127D64EC6_8wekyb3d8bbwe") // TODO: Find a way to get the package name
            .join("LocalState");
        path.push("runtime");
        Ok(path)
        //let mut path = dirs::data_local_dir().unwrap().as_path()
        // Otherwise, try the MSI installation version

        //unimplemented!("Not now, i need to setup a rust dev environment on one of my computers.");
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
