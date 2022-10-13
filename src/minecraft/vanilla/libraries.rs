use std::path::Path;

use futures::{stream::FuturesUnordered, StreamExt};

use crate::{
    config::Config,
    errors::GrindstoneResult,
    event::{EventType, LibraryInstallationUpdate, Progress},
    invoke_callback,
    minecraft::{vanilla::models::version_data::library::Library, VersionData},
    utils::download::Download,
};

impl Library {
    pub async fn install_libraries(
        config: &Config,
        version_data: VersionData,
    ) -> GrindstoneResult<()> {
        let needed_libraries = version_data.needed_libraries();

        let mut count = 0;
        let max = needed_libraries.len() as u32;

        let log_progress = |cur: u32, m: u32, msg: String| {
            invoke_callback!(
                config,
                EventType::Libraries(
                    Progress {
                        current: cur,
                        max: m,
                        message: msg
                    },
                    LibraryInstallationUpdate::Downloading
                ),
                "Downloading libraries"
            );
        };

        let downloads = needed_libraries
            .iter()
            .map(|l| Library::build_download(l, &config.libraries_path()))
            .collect::<Result<Vec<_>, _>>()?;

        let client = reqwest::Client::new();
        let mut tasks = FuturesUnordered::new();

        for d in downloads {
            let a = Download::exec(d, &client);
            tasks.push(a);
        }

        while let Some(res) = tasks.next().await {
            count += 1;
            let name = res?;
            log_progress(count, max, format!("Downloaded library {}", name));
        }

        Ok(())
    }

    fn build_download(
        library: &Library,
        libraries_path: impl AsRef<Path>,
    ) -> GrindstoneResult<Download> {
        let (url, sha1, _size) = library.download_url();

        let sha1 = sha1.as_deref().map(hex::decode).transpose()?;

        Ok(Download {
            url,
            file: library.jar_path(libraries_path),
            sha1,
        })
    }
}
