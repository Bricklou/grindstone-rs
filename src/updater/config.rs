use std::path::{Path, PathBuf};

use crate::errors::{GrindstoneError, GrindstoneResult};

use super::event::CallbackEvent;

pub struct ConfigBuilder {
    event_callack: Box<fn(CallbackEvent)>,
    folder_path: Option<PathBuf>,
    instance_name: Option<String>,
}

#[derive(Debug)]
pub struct Config {
    pub event_callback: Box<fn(CallbackEvent)>,
    pub folder_path: PathBuf,
    pub instance_name: String,
}

impl Default for ConfigBuilder {
    fn default() -> Self {
        Self {
            instance_name: None,
            event_callack: Box::new(|_| {}),
            folder_path: None,
        }
    }
}

impl ConfigBuilder {
    /// Build the config object
    pub fn build(self) -> GrindstoneResult<Config> {
        let instance_name = self
            .instance_name
            .clone()
            .ok_or(GrindstoneError::InvalidConfig("instance_name".to_string()))?;

        let folder_path = self
            .folder_path
            .clone()
            .ok_or(GrindstoneError::InvalidConfig("folder_path".to_string()))?;

        Ok(Config {
            event_callback: self.event_callack,
            folder_path,
            instance_name,
        })
    }

    pub fn name<S: Into<String>>(mut self, instance_name: S) -> Self {
        self.instance_name = Some(instance_name.into());
        self
    }

    /// Set the event callback, it is used to notify progress in the update process.
    ///
    /// * `callback` - The lambda function the library will call to notify progress.
    pub fn set_event_callback(mut self, callback: Box<fn(CallbackEvent)>) -> Self {
        self.event_callack = callback;
        self
    }

    /// Set the output folder path.
    /// * `folder` - Path to the output folder
    pub fn minecraft_folder_path(mut self, folder: impl AsRef<Path>) -> Self {
        self.folder_path = Some(PathBuf::from(folder.as_ref()));
        self
    }
}
