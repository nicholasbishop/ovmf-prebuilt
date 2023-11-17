use crate::error::SyncError;
use crate::prebuilt::{PrebuiltDir, PrebuiltId};
use log::debug;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::{Cursor, Read};
use std::path::PathBuf;
use tar::Archive;
use tempfile::TempDir;
use ureq::Agent;

/// Cache of OVMF prebuilts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrebuiltCache {
    /// Base URL to download prebuilts from.
    ///
    /// Defaults to `https://github.com/rust-osdev/ovmf-prebuilt/releases/download`.
    pub base_url: String,

    /// User agent string sent with download requests.
    ///
    /// Defaults to `https://github.com/rust-osdev/ovmf-prebuilt`.
    pub user_agent: String,

    /// Maximum number of bytes to download per prebuilt tarball.
    ///
    /// Defaults to 8 MiB.
    pub max_download_size_in_bytes: usize,

    /// Base directory in which to cache prebuilts.
    ///
    /// Each prebuilt is downloaded to a subdirectory of this path,
    /// e.g. `<cache_dir>/edk2-stable202308-r1`.
    ///
    /// Defaults to an empty path.
    pub cache_dir: PathBuf,
}

impl PrebuiltCache {
    /// TODO
    pub fn get(&self, prebuilt_id: PrebuiltId) -> Result<PrebuiltDir, SyncError> {
        fs::create_dir_all(&self.cache_dir).map_err(SyncError::CreateCacheDirFailed)?;

        let prebuilt_dir = PrebuiltDir {
            path: self.cache_dir.join(prebuilt_id.tag),
        };

        // If the prebuilt dir exists, assume it is valid.
        if prebuilt_dir.path.exists() {
            return Ok(prebuilt_dir);
        }

        let url = format!(
            "{base}/{tag}/{tag}-bin.tar.xz",
            base = self.base_url,
            tag = prebuilt_id.tag
        );
        let compressed = self
            .download_in_memory(&url)
            .map_err(SyncError::DownloadFailed)?;

        // Validate the hash.
        let actual_hash = format!("{:x}", Sha256::digest(&compressed));
        if actual_hash != prebuilt_id.sha256 {
            return Err(SyncError::HashMismatch {
                actual: actual_hash,
                expected: prebuilt_id.sha256.to_string(),
            });
        }

        debug!("decompressing tarball");
        let mut compressed = Cursor::new(compressed);
        let mut decompressed = Vec::new();
        lzma_rs::xz_decompress(&mut compressed, &mut decompressed)
            .map_err(SyncError::DecompressionFailed)?;

        let unpack_temp_dir =
            TempDir::new_in(&self.cache_dir).map_err(SyncError::CreateTempDirFailed)?;
        debug!(
            "unpacking tarball into {}",
            unpack_temp_dir.path().display()
        );
        let decompressed = Cursor::new(decompressed);
        let mut archive = Archive::new(decompressed);
        archive
            .unpack(unpack_temp_dir.path())
            .map_err(SyncError::UnpackFailed)?;

        let bin_dir = unpack_temp_dir
            .path()
            .join(format!("{}-bin", prebuilt_id.tag));
        debug!(
            "renaming {} to {}",
            bin_dir.display(),
            prebuilt_dir.path.display()
        );
        fs::rename(bin_dir, &prebuilt_dir.path).map_err(SyncError::RenameFailed)?;

        Ok(prebuilt_dir)
    }

    /// Download `url` and return the raw data.
    ///
    /// The download is done via `ureq`. The user agent is set to
    /// `self.user_agent`. The download size is limited to
    /// `self.max_download_size_in_bytes`.
    fn download_in_memory(&self, url: &str) -> Result<Vec<u8>, ureq::Error> {
        let agent: Agent = ureq::AgentBuilder::new()
            .user_agent(&self.user_agent)
            .build();

        // Download the file.
        debug!("downloading {url}");
        let resp = agent.get(url).call()?;
        let mut data = Vec::with_capacity(self.max_download_size_in_bytes);
        resp.into_reader()
            .take(self.max_download_size_in_bytes.try_into().unwrap())
            .read_to_end(&mut data)?;
        debug!("received {} bytes", data.len());

        Ok(data)
    }
}

impl Default for PrebuiltCache {
    fn default() -> Self {
        Self {
            base_url: "https://github.com/rust-osdev/ovmf-prebuilt/releases/download".to_string(),
            user_agent: "https://github.com/rust-osdev/ovmf-prebuilt".to_string(),
            max_download_size_in_bytes: 8 * 1024 * 1024,
            cache_dir: PathBuf::new(),
        }
    }
}
