use std::path::PathBuf;

use crate::config::Config;

impl Config {
    /// Path to the updater folder
    /// This path contains all the files which cannot be placed inside the .minecraft folder
    pub fn updater_folder(&self) -> PathBuf {
        PathBuf::from(&self.folder_path)
    }

    /// Path to the instance folder
    pub fn current_instance(&self) -> PathBuf {
        let mut path = self.updater_folder();
        path.push("instances");
        path.push(self.instance_name.clone());
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
    /// Version are shared with the official Minecraft launcher
    pub fn versions_path(&self) -> PathBuf {
        let mut path = self.dot_minecraft_path();
        path.push("versions");
        path
    }

    /// Path to the version JAR file
    pub fn version_jar_path(&self) -> PathBuf {
        let mut path = self.versions_path();
        let v = self.version.id.clone();
        path.push(&v);
        path.push(format!("{}.jar", v));
        path
    }

    /// Path to the version manifest file
    pub fn version_data_path(&self) -> PathBuf {
        let mut path = self.versions_path();
        let v = self.version.id.clone();
        path.push(&v);
        path.push(format!("{}.json", v));
        path
    }

    /// Path to the game assets.
    pub fn assets_path(&self) -> PathBuf {
        let mut path = self.dot_minecraft_path();
        path.push("assets");
        path
    }

    /// Path to the asset index JSON file.
    pub fn asset_index_path(&self) -> PathBuf {
        let mut path = self.assets_path();
        path.push("indexes");
        path
    }

    /// Path to the game libraries
    /// Libraries are shared with the official Minecraft launcher
    pub fn libraries_path(&self) -> PathBuf {
        let mut path = self.dot_minecraft_path();
        path.push("libraries");
        path
    }

    /// Path to the game libraries
    /// Libraries are shared with the official Minecraft launcher
    pub fn natives_path(&self) -> PathBuf {
        let mut path = self.dot_minecraft_path();
        path.push("libraries");
        path
    }

    /// Path to the logs config XML file
    pub fn log_configs_path(&self) -> PathBuf {
        let mut path = self.assets_path();
        path.push("log_configs");
        path
    }
}
