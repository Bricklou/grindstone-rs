use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
};

use log::warn;
use serde::{Deserialize, Serialize};

use crate::{constants, utils::os::Architecture};

use super::{extract::Extract, library_download::LibraryDownloads, natives::Natives, rules::Rule};

/// A library need for running the game.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Library {
    /// Information about downloading the library.
    pub downloads: LibraryDownloads,
    /// Name of the library.
    pub name: String,
    /// Available natives.
    pub natives: Option<Natives>,
    /// Rules for this library.
    #[serde(default)]
    pub rules: Vec<Rule>,
    /// Extract options.
    pub extract: Option<Extract>,
}

impl Library {
    /// Checks if the library needs to be used on the current machine.
    pub fn check_use(&self) -> bool {
        for rule in &self.rules {
            if !rule.allows() {
                return false;
            }
        }

        true
    }

    /// Builds the name of the library jar file.
    pub fn jar_name(&self) -> String {
        let split = self.split_name();
        // Take name and version
        let parts = split.iter().skip(1).take(2).cloned();
        // Take suffixes
        let suffixes = split.iter().skip(3).cloned();

        let mut name = vec![];

        // Name & version
        name.extend(parts);

        // Native
        if let Some(native) = self.get_native() {
            name.push(native);
        }

        // Suffixes
        name.extend(suffixes);

        let mut name = name.join("-");
        name.push_str(".jar");

        name
    }

    /// Builds the path where the library jar file is placed.
    /// This path does not include the file itself.
    pub fn library_path(&self, libraries_path: impl AsRef<Path>) -> PathBuf {
        // Take package, name and version
        let path_parts = self.split_name().into_iter().take(3);

        let mut library_path = PathBuf::from(libraries_path.as_ref());
        for (i, path_part) in path_parts.enumerate() {
            let part = match i {
                0 => path_part.replace('.', "/"),
                _ => path_part,
            };

            library_path.push(part);
        }

        library_path
    }

    /// Builds the complete path for the library jar file.
    pub fn jar_path(&self, libraries_path: impl AsRef<Path>) -> PathBuf {
        let mut jar_path = self.library_path(&libraries_path);
        jar_path.push(self.jar_name());

        jar_path
    }

    /// Builds the download URL for the library.
    /// Returns a tuble: (URL, SHA1, size).
    pub fn download_url(&self) -> (String, Option<String>, Option<usize>) {
        let url: String;
        let mut sha1 = None;
        let mut size = None;

        match &self.get_native() {
            Some(native) => match self.downloads.classifiers.get(native) {
                Some(file) => {
                    url = file.url.clone();
                    sha1 = Some(file.sha1.clone());
                    size = Some(file.size);
                }
                None => {
                    url = self.build_url_from_name();
                }
            },
            None => match &self.downloads.artifact {
                Some(file) => {
                    url = file.url.clone();
                    sha1 = Some(file.sha1.clone());
                    size = Some(file.size);
                }
                None => {
                    url = self.build_url_from_name();
                }
            },
        }

        (url, sha1, size)
    }

    /// Checks if the library needs to be extracted.
    pub fn needs_extract(&self) -> bool {
        self.get_native().is_some() && self.extract.is_some()
    }

    fn build_url_from_name(&self) -> String {
        // Take package, name and version
        let parts = self.split_name().into_iter().take(3);

        let mut url = vec![constants::MC_LIBRARIES_BASE_URL.to_string()];

        for (i, part) in parts.enumerate() {
            if i == 0 {
                url.push(part.replace('.', "/"));
            } else {
                url.push(part);
            }
        }

        url.push(self.jar_name());

        url.join("/")
    }

    /// Gets the native identifier of the library.
    pub fn get_native(&self) -> Option<String> {
        let arch = Architecture::current();

        self.natives.as_ref().and_then(|natives| {
            natives
                .get_for_current_platform()
                .map(|n| n.replace("${arch}", &arch.get_bits().to_string()))
        })
    }

    fn split_name(&self) -> Vec<String> {
        self.name.split(':').map(String::from).collect()
    }
}
