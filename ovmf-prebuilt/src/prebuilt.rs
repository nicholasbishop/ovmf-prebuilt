use std::path::PathBuf;

/// A release package from <https://github.com/rust-osdev/ovmf-prebuilt/releases>.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PrebuiltId<'a> {
    /// Name of the release tag, e.g. `edk2-stable202308-r1`.
    pub tag: &'a str,

    /// SHA-256 hex digest of the compressed tarball.
    pub sha256: &'a str,
}

// Allow missing docs, nothing useful to add for these constants.
#[allow(missing_docs)]
impl<'a> PrebuiltId<'a> {
    pub const EDK2_STABLE202402_R1: Self = Self {
        tag: "edk2-stable202402-r1",
        sha256: "91f3148ef146794241c77810a49cfa3e925c83eb55c5cc90f34718cc1b10e9eb",
    };

    pub const EDK2_STABLE202311_R2: Self = Self {
        tag: "edk2-stable202311-r2",
        sha256: "4a7d01b7dc6b0fdbf3a0e17dacd364b772fb5b712aaf64ecf328273584185ca0",
    };

    pub const EDK2_STABLE202311_R1: Self = Self {
        tag: "edk2-stable202311-r1",
        sha256: "2587ddd6b0134ecee122f9772aa8e40cd3765f3c1b7b453a56543f29f1e184eb",
    };

    pub const EDK2_STABLE202308_R1: Self = Self {
        tag: "edk2-stable202308-r1",
        sha256: "e75df3424e1c8edf9a6c14027a5a9dd16201d66cde0ad86766ccdc58aeebcccf",
    };

    pub const EDK2_STABLE202305_R1: Self = Self {
        tag: "edk2-stable202305-r1",
        sha256: "644a5a5aee748cd3d06e403a1b2c6bce934f8325122d3ecd365f2bc99d9d2016",
    };

    pub const EDK2_STABLE202302_R1: Self = Self {
        tag: "edk2-stable202302-r1",
        sha256: "1d9a30afbf6a07c6580ca67629ea68c01e8449ef93c2e40482081f04b6f06ddb",
    };

    pub const EDK2_STABLE202211_R1: Self = Self {
        tag: "edk2-stable202211-r1",
        sha256: "b085cfe18fd674bf70a31af1dc3e991bcd25cb882981c6d3523d81260f1e0d12",
    };
}

/// Local directory containing prebuilt artifacts.
#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PrebuiltDir {
    /// Base directory.
    pub path: PathBuf,
}

impl PrebuiltDir {
    /// Get the path to an artifact for a given arch.
    pub fn get(&self, arch: Arch, artifact: Artifact) -> PathBuf {
        self.path.join(arch.dir_name()).join(artifact.file_name())
    }
}

/// Build architecture of a prebuilt artifact.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Arch {
    /// Intel IA-32.
    Ia32,

    /// Intel x64.
    X64,

    /// ARM AArch64.
    Aarch64,

    /// RISC-V 64-bit.
    Riscv64,
}

impl Arch {
    /// Get all architectures.
    ///
    /// Note that not all prebuilts are guaranteed to have all
    /// architectures included.
    pub const fn all() -> &'static [Self] {
        &[Self::Ia32, Self::X64, Self::Aarch64, Self::Riscv64]
    }

    /// Get the directory name for the architecture.
    pub const fn dir_name(&self) -> &'static str {
        match self {
            Self::Ia32 => "ia32",
            Self::X64 => "x64",
            Self::Aarch64 => "aarch64",
            Self::Riscv64 => "riscv64",
        }
    }
}

/// Specific type of prebuilt artifact.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Artifact {
    /// Firmware code file.
    Code,

    /// Firmware vars file.
    Vars,

    /// UEFI shell executable.
    Shell,
}

impl Artifact {
    /// Get all artifact types.
    ///
    /// Note that not all prebuilts are guaranteed to have all artifact
    /// types included.
    pub const fn all() -> &'static [Self] {
        &[Self::Code, Self::Vars, Self::Shell]
    }

    /// Get the file name for the artifact type.
    pub const fn file_name(&self) -> &'static str {
        match self {
            Self::Code => "code.fd",
            Self::Vars => "vars.fd",
            Self::Shell => "shell.efi",
        }
    }
}
