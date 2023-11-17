use std::fmt::{self, Display, Formatter};
use std::io;

/// Error returned by [`Cache::sync`].
///
/// [`Cache::sync`]: crate::Cache::sync
#[derive(Debug)]
pub enum SyncError {
    /// Failed to create cache directory.
    CreateCacheDirFailed(io::Error),

    /// Failed to download prebuilt tarball.
    DownloadFailed(ureq::Error),

    /// The downloaded prebuilt tarball's hash does not match the expected hash.
    HashMismatch {
        /// SHA-256 hex digest of the download.
        actual: String,
        /// Expected SHA-256 hex digest of the prebuilt.
        expected: String,
    },

    /// Failed to decompress the prebuilt tarball.
    DecompressionFailed(lzma_rs::error::Error),

    /// Failed to create a temporary directory.
    CreateTempDirFailed(io::Error),

    /// Failed to unpack the prebuilt tarball.
    UnpackFailed(io::Error),

    /// Failed to rename a directory.
    RenameFailed(io::Error),
}

impl Display for SyncError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreateCacheDirFailed(err) => write!(f, "failed to create cache directory: {err}"),
            Self::DownloadFailed(err) => write!(f, "download failed: {err}"),
            Self::HashMismatch { actual, expected } => {
                write!(f, "expected hash {expected}, got hash {actual}")
            }
            Self::DecompressionFailed(err) => write!(f, "decompression failed: {err}"),
            Self::CreateTempDirFailed(err) => {
                write!(f, "failed to create temporary directory: {err}")
            }
            Self::UnpackFailed(err) => write!(f, "failed to unpack tarball: {err}"),
            Self::RenameFailed(err) => write!(f, "failed to rename directory: {err}"),
        }
    }
}

impl std::error::Error for SyncError {}
