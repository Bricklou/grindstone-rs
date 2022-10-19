use std::{collections::HashMap, fs, path::PathBuf};

use futures::{stream::FuturesUnordered, StreamExt};
use log::{debug, trace};
use serde::Deserialize;

use crate::{
    errors::GrindstoneResult,
    event::{EventType, Progress},
    invoke_callback,
    utils::download::download_file_check,
};

use super::{
    runtime_manifest::{FileType, JreFile},
    Java,
};

#[derive(Deserialize, Debug)]
pub struct DownloadFile {
    pub sha1: String,
    pub size: usize,
    pub url: String,
}

impl Java {
    pub async fn download_jre_files(
        &self,
        dest: &PathBuf,
        files: HashMap<String, JreFile>,
    ) -> GrindstoneResult<()> {
        debug!("No JRE found, downloading one");

        //let mut v = Vec::new();
        let mut links = HashMap::<String, JreFile>::new();
        let mut exec = Vec::<PathBuf>::new();

        let mut count = 0;
        let max = files.len() as u32;

        trace!("Downloading JRE files");

        let log_progress = |cur: u32, m: u32, msg: String| {
            invoke_callback!(
                self.config,
                EventType::DownloadJRE(Progress {
                    current: cur,
                    max: m,
                    message: msg
                }),
                "Downloading JRE"
            );
        };

        let client = reqwest::Client::new();

        let mut tasks = FuturesUnordered::new();

        for (path, data) in files {
            match data.file_type {
                FileType::Directory => {
                    let path = dest.join(path);
                    fs::create_dir_all(path)?;
                    count += 1;
                    log_progress(count, max, "Creating folders".to_string());
                }
                FileType::File => {
                    let path = dest.join(path);
                    let parent_dir = path.parent().unwrap();
                    fs::create_dir_all(parent_dir)?;

                    let raw_data = data.downloads.unwrap().raw;
                    let sha = hex::decode(&raw_data.sha1)?;

                    let a =
                        download_file_check(&client, raw_data.url.clone(), path.clone(), Some(sha));
                    tasks.push(a);

                    if data.executable == Some(true) {
                        exec.push(path);
                    }
                }
                FileType::Link => {
                    links.insert(path, data);
                }
            }
        }

        while let Some(res) = tasks.next().await {
            count += 1;
            let name = res?;
            log_progress(count, max, format!("Downloaded file {}", name));
        }

        trace!("Creating missing symbolic links");
        for (path, data) in links {
            let path = dest.join(path);
            let target = PathBuf::from(data.target.unwrap());

            count += 1;
            log_progress(
                count,
                max,
                format!(
                    "Creating symbolic links {}",
                    target.as_path().to_string_lossy()
                ),
            );

            if path.exists() {
                if !path.is_symlink() || path.read_link()? != target {
                    fs::remove_file(&path)?;
                } else {
                    continue;
                }
            }

            cfg_if::cfg_if! {
                if #[cfg(unix)] {
                    std::os::unix::fs::symlink(target, path)?;
                } else if #[cfg(target_os = "windows")] {
                    std::os::windows::fs::symlink_file(target, path)?;
                } else {
                    compile_error!("Unknown platform {}", env::consts::OS)
                }
            }
        }

        #[cfg(unix)]
        {
            use std::os::{linux::raw, unix::prelude::PermissionsExt};

            trace!("Apply files permissions");
            for path in exec {
                let mut perms = fs::metadata(&path)?.permissions();
                perms.set_mode(perms.mode() | 0o700);
                fs::set_permissions(path, perms)?
            }
        }
        Ok(())
    }
}
