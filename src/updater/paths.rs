use std::path::PathBuf;

use crate::GrindstoneUpdater;

impl GrindstoneUpdater {
    /// Path to the updater folder
    /// This path contains all the files which cannot be placed inside the .minecraft folder
    pub fn updater_folder(&self) -> PathBuf {
        PathBuf::from(&self.config.folder_path)
    }

    /// Path to the instance folder
    pub fn current_instance(&self) -> PathBuf {
        let mut path = self.updater_folder();
        path.push("instances");
        path.push(self.config.instance_name.clone());
        path
    }

    /// Path to the .minecraft folder
    pub fn dot_minecraft_path(&self) -> PathBuf {
        use cfg_if::cfg_if;

        cfg_if! {
            if #[cfg(windows)] {
                let path = dirs::data_dir().unwrap();

            } else if #[cfg(any(unix,macos))] {
                let mut path = dirs::home_dir().unwrap();
            } else {
                compile_error!("Unknow platform {}", env::consts::OS)
            }
        }

        path.push(".minecraft");
        path
    }

    /// Path to the game versions
    /// Version are shared with the offical minecraft launcher
    pub fn versions_path(&self) -> PathBuf {
        let mut path = self.dot_minecraft_path();
        path.push("versions");
        path
    }

    /// Path to the version manifest file
    pub fn version_data_path(&self) -> PathBuf {
        let mut path = self.versions_path();
        let v = self.config.version.id.clone();
        path.push(&v);
        path.push(format!("{}.json", v));
        path
    }
}
