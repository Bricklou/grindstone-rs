use log::trace;
use std::{fs::File, io::Cursor, path::Path};

use crate::errors::{GrindstoneError, GrindstoneResult};

use super::sha1::get_sha1;

pub async fn download_file<S: Into<String>>(
    url: S,
    dest: impl AsRef<Path>,
) -> GrindstoneResult<()> {
    let url = url.into();
    trace!("Downloading file: {}", url);

    let response = reqwest::get(url).await?.error_for_status()?;

    let mut file = File::create(&dest)?;

    let mut content = Cursor::new(response.bytes().await?);
    let _total_size = std::io::copy(&mut content, &mut file)?;
    file.sync_all()?;

    Ok(())
}

pub async fn download_file_check<S: Into<String>>(
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

                    download_file(&url, &dest).await?;
                }
            }
        }
    } else {
        download_file(&url, &dest).await?;
    }

    if let Some(remote_sha) = &remote_sha {
        let local_sha = get_sha1(&dest)?;
        if remote_sha != &local_sha {
            return Err(GrindstoneError::ChecksumMismatch);
        }
    }

    Ok(url)
}
