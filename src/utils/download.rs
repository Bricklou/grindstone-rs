use futures::StreamExt;
use log::trace;
use std::{
    fs::File,
    io::Cursor,
    path::{Path, PathBuf},
};

use tokio::io::AsyncWriteExt;

use crate::errors::{GrindstoneError, GrindstoneResult};

use super::sha1::get_sha1;

#[derive(Clone, Debug)]
pub struct Download {
    pub url: String,
    pub file: PathBuf,
    pub sha1: Option<Vec<u8>>,
}

impl Download {
    pub async fn exec(d: Self, client: &reqwest::Client) -> GrindstoneResult<String> {
        use tokio::fs::{create_dir_all, File};

        // Create parent folder
        if let Some(parent) = d.file.parent() {
            trace!("Creating parent folder");
            create_dir_all(parent).await?;
        }

        trace!("Downloading file: {}", d.url);

        let response = client.get(&d.url).send().await?.error_for_status()?;

        let mut file = File::create(&d.file).await?;
        let mut stream = response.bytes_stream();

        while let Some(item) = stream.next().await {
            let chunk = item?;
            file.write_all(&chunk).await?;
        }

        file.sync_all().await?;

        Ok(d.url.clone())
    }
}

pub async fn download_file<S: Into<String>>(
    client: &reqwest::Client,
    url: S,
    dest: impl AsRef<Path>,
) -> GrindstoneResult<()> {
    let url = url.into();
    trace!("Downloading file: {}", url);

    let response = client.get(url).send().await?.error_for_status()?;

    let mut file = File::create(&dest)?;

    let mut content = Cursor::new(response.bytes().await?);
    let _total_size = std::io::copy(&mut content, &mut file)?;
    file.sync_all()?;

    Ok(())
}

pub async fn download_file_check<S: Into<String>>(
    client: &reqwest::Client,
    url: S,
    dest: impl AsRef<Path>,
    remote_sha: Option<Vec<u8>>,
) -> GrindstoneResult<String> {
    let url = url.into();
    trace!("Checked download of file: {}", url);

    if dest.as_ref().exists() {
        trace!("File already exists");

        match &remote_sha {
            None => {
                return Ok(url);
            }
            Some(remote_sha) => {
                let local_sha = get_sha1(&dest)?;

                if remote_sha == &local_sha {
                    trace!("Existing file is correct");

                    return Ok(url);
                } else {
                    trace!("Existing file does not match checksum");

                    download_file(&client, &url, &dest).await?;
                }
            }
        }
    } else {
        download_file(&client, &url, &dest).await?;
    }

    if let Some(remote_sha) = &remote_sha {
        let local_sha = get_sha1(&dest)?;
        if remote_sha != &local_sha {
            return Err(GrindstoneError::ChecksumMismatch);
        }
    }

    Ok(url)
}

/// Progress of an ongoing download.
#[derive(Clone, Debug)]
pub struct DownloadProgress {
    /// The URL of the download.
    pub url: String,
    /// The path where the file is saved.
    pub file: PathBuf,
    /// Current file index.
    pub current_file: usize,
    /// Number of files that are being downloaded.
    pub total_files: usize,
    /// Bytes that already got downloaded.
    pub downloaded_bytes: u64,
    /// Total bytes of the file.
    pub total_bytes: u64,
}
