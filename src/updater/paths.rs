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
}
