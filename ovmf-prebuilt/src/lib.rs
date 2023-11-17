#![warn(missing_docs)]
// This code is not performance-sensitive, no need to box errors.
#![allow(clippy::large_enum_variant, clippy::result_large_err)]

//! TODO
//!
//! ```no_run
//! use std::path::PathBuf;
//! use ovmf_prebuilt::{Arch, Artifact, PrebuiltCache, PrebuiltId};
//!
//! let cache = Cache {
//!   cache_dir: PathBuf::from("ovmf-cache"),
//!   ..Default::default()
//! };
//! let prebuilt_dir = cache.get(PrebuiltId::EDK2_STABLE202308_R1).unwrap();
//! let artifact_path = prebuilt_dir.get(Arch::X64, Artifact::Code);
//! ```

mod cache;
mod error;
mod prebuilt;

pub use cache::PrebuiltCache;
pub use error::SyncError;
pub use prebuilt::{Arch, Artifact, PrebuiltDir, PrebuiltId};

// TODO: Integration
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync() {
        let temp_dir = tempfile::tempdir().unwrap();

        let config = PrebuiltCache {
            cache_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };
        config.get(PrebuiltId::EDK2_STABLE202211_R1).unwrap();
    }
}
