use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustcInfo {
    /// Root directory where the Rust compiler looks for standard libraries and other
    /// essential components when building Rust projects.
    pub sysroot_path: URI,
    /// Source code for the Rust standard library.
    pub src_sysroot_path: URI,
    /// `rustc` SemVer (Semantic Versioning) version.
    pub version: String,
    /// Target architecture and operating system of the Rust compiler.
    /// Used by [`intellij-rust`] for checking if given toolchain is supported.
    pub host: String,
}
