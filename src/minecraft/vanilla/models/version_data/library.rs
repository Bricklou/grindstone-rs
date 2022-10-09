use std::{
    collections::VecDeque,
    path::{Path, PathBuf},
};

use log::warn;
use serde::{Deserialize, Serialize};

use crate::{
    constants,
    errors::{GrindstoneError, GrindstoneResult},
    utils::os::Architecture,
};

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
    /// Checks if the library needs to be used on the current executing machine.
    pub fn check_use(&self) -> bool {
        for rule in &self.rules {
            if !rule.allows() {
                return false;
            }
        }

        true
    }

    /// Builds the path where the library jar file is placed.
    /// This path does not include the jar file itself.
    pub fn library_path(&self, libraries_path: impl AsRef<Path>) -> GrindstoneResult<PathBuf> {
        let mut split = self.split_name();
        let mut package = split
            .pop_front()
            .ok_or(GrindstoneError::LibraryNameFormat)?;
        let name = split
            .pop_front()
            .ok_or(GrindstoneError::LibraryNameFormat)?;
        let version = split
            .pop_front()
            .ok_or(GrindstoneError::LibraryNameFormat)?;

        package = package.replace('.', "/");

        let mut library_path = PathBuf::from(libraries_path.as_ref());
        library_path.push(&package);
        library_path.push(&name);
        library_path.push(&version);

        Ok(library_path)
    }

    /// Build the path for the library jar file.
    /// Supports building the file name for natives.
    /// This path does include the jar file itself.
    pub fn jar_path(
        &self,
        libraries_path: impl AsRef<Path>,
        native: Option<&str>,
    ) -> GrindstoneResult<PathBuf> {
        let mut split = self.split_name();
        let mut package = split
            .pop_front()
            .ok_or(GrindstoneError::LibraryNameFormat)?;
        let name = split
            .pop_front()
            .ok_or(GrindstoneError::LibraryNameFormat)?;
        let version = split
            .pop_front()
            .ok_or(GrindstoneError::LibraryNameFormat)?;
        let suffix = split.pop_front();

        package = package.replace('.', "/");

        let mut library_path = PathBuf::from(libraries_path.as_ref());
        library_path.push(&package);
        library_path.push(&name);
        library_path.push(&version);

        let jar_name = Self::jar_name(&name, &version, native, suffix.as_deref());

        library_path.push(jar_name);

        Ok(library_path)
    }

    /// Gets the download URL for this library.
    /// Supports download URL for native file by providing the native identifier.
    /// Returns the URL, SHA1 and file size.
    pub fn download_url(
        &self,
        native: Option<&str>,
    ) -> GrindstoneResult<(String, Option<String>, Option<usize>)> {
        let url: String;
        let mut sha1 = None;
        let mut size = None;

        match native {
            Some(native) => {
                match self.downloads.classifiers.get(native) {
                    // Take data from file
                    Some(file) => {
                        url = file.url.clone();
                        sha1 = Some(file.sha1.clone());
                        size = Some(file.size);
                    }
                    // Build data from name
                    None => {
                        url = self.build_url_from_name(Some(native))?;
                    }
                }
            }
            None => {
                match &self.downloads.artifact {
                    // Take data from file
                    Some(file) => {
                        url = file.url.clone();
                        sha1 = Some(file.sha1.clone());
                        size = Some(file.size);
                    }
                    // Build data from name
                    None => {
                        url = self.build_url_from_name(None)?;
                    }
                }
            }
        }

        Ok((url, sha1, size))
    }

    fn build_url_from_name(&self, native: Option<&str>) -> GrindstoneResult<String> {
        let mut split = self.split_name();
        let mut package = split
            .pop_front()
            .ok_or(GrindstoneError::LibraryNameFormat)?;
        let name = split
            .pop_front()
            .ok_or(GrindstoneError::LibraryNameFormat)?;
        let version = split
            .pop_front()
            .ok_or(GrindstoneError::LibraryNameFormat)?;

        package = package.replace('.', "/");

        let url = format!(
            "{}/{}/{}/{}/{}",
            constants::MC_LIBRARIES_BASE_URL,
            &package,
            &name,
            &version,
            &Self::jar_name(&name, &version, native, None),
        );

        warn!("{}", &url);

        Ok(url)
    }

    /// Build the name for the library jar file.
    fn jar_name(name: &str, version: &str, native: Option<&str>, suffix: Option<&str>) -> String {
        match (suffix, native) {
            (Some(suffix), Some(native)) => {
                format!("{}-{}-{}-{}.jar", name, version, native, suffix)
            }
            (Some(suffix), None) => format!("{}-{}-{}.jar", name, version, suffix),
            (None, Some(native)) => format!("{}-{}-{}.jar", name, version, native),
            (None, None) => format!("{}-{}.jar", name, version),
        }
    }

    /// Splits the library name at its delimitor (`:`).
    fn split_name(&self) -> VecDeque<String> {
        self.name
            .split(':')
            .map(|x| x.to_string())
            .collect::<VecDeque<_>>()
    }

    /// Gets the native library if applicable.
    pub fn get_native(&self) -> Option<String> {
        let arch = Architecture::current();

        if let Some(natives) = &self.natives {
            return natives
                .get_for_current_platform()
                .map(|n| n.replace("${arch}", &arch.get_bits().to_string()));
        }

        None
    }
}
