use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustToolchainItem {
    /// Additional information about Rust toolchain.
    /// Obtained from `rustc`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rust_std_lib: Option<RustcInfo>,
    /// Path to Cargo executable.
    pub cargo_bin_path: URI,
    /// Location of the source code of procedural macros in the Rust toolchain.
    pub proc_macro_srv_path: URI,
}
