use serde::{Deserialize, Serialize};

use crate::*;

/** This structure is embedded in the `data?: BuildTargetData` field, when the
`dataKind` field contains "rust". */
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustBuildTarget {
    /** The name of the target. */
    #[serde(default)]
    pub name: String,
    /** Path to the root module of the crate. */
    #[serde(default)]
    pub crate_root_url: URI,
    /** A target's kind. */
    #[serde(default)]
    pub kind: RustTargetKind,
    /** Type of output that is produced by a crate during the build process.
    The crate type determines how the source code is compiled. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub crate_types: Vec<RustCrateType>,
    /** The Rust edition of the target. */
    #[serde(default)]
    pub edition: RustEdition,
    /** Whether or not this target has doc tests enabled, and
    the target is compatible with doc testing. */
    #[serde(default)]
    pub doctest: bool,
    /** A sequence of required features. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_features: Vec<String>,
}
